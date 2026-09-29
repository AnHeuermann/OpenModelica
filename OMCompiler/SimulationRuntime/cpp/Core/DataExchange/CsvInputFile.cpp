/*
 * This file belongs to the OpenModelica Run-Time System
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC), c/o Linköpings
 * universitet, Department of Computer and Information Science, SE-58183 Linköping, Sweden. All rights
 * reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF THE BSD NEW LICENSE OR THE
 * AGPL VERSION 3 LICENSE OR THE OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8. ANY
 * USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S
 * ACCEPTANCE OF THE BSD NEW LICENSE OR THE OSMC PUBLIC LICENSE OR THE AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium) Public License
 * (OSMC-PL) are obtained from OSMC, either from the above address, from the URLs:
 * http://www.openmodelica.org or https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica, and in the OpenModelica distribution. GNU
 * AGPL version 3 is obtained from: https://www.gnu.org/licenses/licenses.html#GPL. The BSD NEW
 * License is obtained from: http://www.opensource.org/licenses/BSD-3-Clause.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY
 * SET FORTH IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF
 * OSMC-PL.
 *
 */

/** @addtogroup dataexchange
 *
 *  @{
 */
#include <Core/ModelicaDefine.h>
#include <Core/Modelica.h>
#include <Core/DataExchange/FactoryExport.h>
#include <Core/DataExchange/CsvInputFile.h>
#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <fstream>
#include <sstream>

/**
 * Split one CSV record into its fields. Fields may be enclosed in double
 * quotes, with "" for a quote inside, and then contain the separator. Line
 * breaks inside quoted fields are not supported.
 */
static std::vector<std::string> splitRecord(const std::string& line, char sep)
{
  std::vector<std::string> fields;
  std::string field;
  bool quoted = false;
  for (size_t i = 0; i < line.size(); i++)
  {
    char c = line[i];
    if (quoted)
    {
      if (c == '"' && i + 1 < line.size() && line[i + 1] == '"')
      {
        field += '"';
        i++;
      }
      else if (c == '"')
        quoted = false;
      else
        field += c;
    }
    else if (c == '"')
      quoted = true;
    else if (c == sep)
    {
      fields.push_back(field);
      field.clear();
    }
    else if (c != '\r')
      field += c;
  }
  fields.push_back(field);
  return fields;
}

CsvInputFile::CsvInputFile(const std::string& fileName)
{
  read(fileName);
}

CsvInputFile::~CsvInputFile()
{
}

void CsvInputFile::read(const std::string& fileName)
{
  std::ifstream file(fileName.c_str());
  if (!file.good())
    throw ModelicaSimulationError(DATASTORAGE, "Failed to open input file " + fileName);

  std::string line;
  char sep = ',';
  bool haveHeader = false;
  size_t nColumns = 0;
  size_t row = 0;
  while (std::getline(file, line))
  {
    if (!line.empty() && line[line.size() - 1] == '\r')
      line.erase(line.size() - 1);
    if (!haveHeader)
    {
      // optional first line "sep=X" or "\"sep=X\"" selecting the separator
      std::string l = line;
      l.erase(std::remove(l.begin(), l.end(), '"'), l.end());
      if (row == 0 && l.size() == 5 && l.compare(0, 4, "sep=") == 0)
      {
        sep = l[4];
        row++;
        continue;
      }
      std::vector<std::string> names = splitRecord(line, sep);
      nColumns = names.size();
      if (nColumns < 1)
        throw ModelicaSimulationError(DATASTORAGE, "Input file " + fileName + " has no columns");
      for (size_t k = 1; k < nColumns; k++)
        _columns[names[k]] = k - 1;
      _values.resize(nColumns - 1);
      haveHeader = true;
      row++;
      continue;
    }
    row++;
    if (line.find_first_not_of(" \t") == std::string::npos)
      continue;
    std::vector<std::string> fields = splitRecord(line, sep);
    if (fields.size() != nColumns)
    {
      std::stringstream ss;
      ss << "Input file " << fileName << ": row " << row << " has " << fields.size() << " values, expected " << nColumns;
      throw ModelicaSimulationError(DATASTORAGE, ss.str());
    }
    for (size_t k = 0; k < nColumns; k++)
    {
      const char* s = fields[k].c_str();
      char* end = NULL;
      double v = fields[k].empty() ? 0.0 : std::strtod(s, &end);
      if (!fields[k].empty() && (end == s || *end != '\0'))
      {
        std::stringstream ss;
        ss << "Input file " << fileName << ": row " << row << " has non-numeric value \"" << fields[k] << "\"";
        throw ModelicaSimulationError(DATASTORAGE, ss.str());
      }
      if (k == 0)
        _time.push_back(v);
      else
        _values[k - 1].push_back(v);
    }
  }
  if (!haveHeader)
    throw ModelicaSimulationError(DATASTORAGE, "Input file " + fileName + " is empty");
}

bool CsvInputFile::add(const std::string& name, InputType type, void* var)
{
  std::map<std::string, size_t>::const_iterator it = _columns.find(name);
  if (it == _columns.end() || _time.empty())
    return false;
  Binding b = {it->second, type, var};
  _bindings.push_back(b);
  return true;
}

bool CsvInputFile::addReal(const std::string& name, double* var)
{
  return add(name, REAL_INPUT, var);
}

bool CsvInputFile::addInt(const std::string& name, int* var)
{
  return add(name, INT_INPUT, var);
}

bool CsvInputFile::addBool(const std::string& name, bool* var)
{
  return add(name, BOOL_INPUT, var);
}

void CsvInputFile::apply(double t)
{
  if (_bindings.empty())
    return;
  const size_t n = _time.size();
  // last row with time <= t; the first row for t before the table
  size_t last = std::upper_bound(_time.begin(), _time.end(), t) - _time.begin();
  last = last > 0 ? last - 1 : 0;
  // interval used for the interpolation, extrapolating with the first or last one
  size_t i = n < 2 ? 0 : std::min(last, n - 2);
  for (size_t b = 0; b < _bindings.size(); b++)
  {
    const std::vector<double>& u = _values[_bindings[b].column];
    double v;
    if (_bindings[b].type != REAL_INPUT || n < 2)
      v = u[last];
    else
    {
      double t1 = _time[i], t2 = _time[i + 1];
      double u1 = u[i], u2 = u[i + 1];
      if (t == t1 || u1 == u2 || t2 == t1)
        v = u1;
      else if (t == t2)
        v = u2;
      else
        v = (u1 * (t2 - t) + u2 * (t - t1)) / (t2 - t1);
    }
    switch (_bindings[b].type)
    {
      case REAL_INPUT:
        *static_cast<double*>(_bindings[b].var) = v;
        break;
      case INT_INPUT:
        *static_cast<int*>(_bindings[b].var) = (int)std::floor(v + 0.5);
        break;
      case BOOL_INPUT:
        *static_cast<bool*>(_bindings[b].var) = v != 0.0;
        break;
    }
  }
}
/** @} */ // end of dataexchange
