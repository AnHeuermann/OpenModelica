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

#pragma once
/** @addtogroup dataexchange
 *
 *  @{
 */

#include <map>
#include <string>
#include <vector>

/**
 * Inputs from a CSV file, compatible with -csvInput of the C runtime.
 *
 * The first row holds the column names, the first column the time.
 * A first line "sep=X" (optionally quoted) selects the separator X instead of
 * ','. Names may be quoted, e.g. "u[1,2]" when the separator is ','.
 *
 * Real inputs are interpolated linearly between the rows, and extrapolated
 * linearly with the first or last interval outside of them, like the C
 * runtime. Integer and Boolean inputs keep the value of the last row at or
 * before the time. Inputs without a column keep their start value.
 */
class BOOST_EXTENSION_XML_READER_DECL CsvInputFile : public IInputFile
{
  public:
    CsvInputFile(const std::string& fileName);
    virtual ~CsvInputFile();

    /// Connect input element `name` to its storage; false if the file has no such column
    bool addReal(const std::string& name, double* var);
    bool addInt(const std::string& name, int* var);
    bool addBool(const std::string& name, bool* var);

    virtual void apply(double t);

  private:
    enum InputType { REAL_INPUT, INT_INPUT, BOOL_INPUT };
    struct Binding
    {
      size_t column;
      InputType type;
      void* var;
    };

    bool add(const std::string& name, InputType type, void* var);
    void read(const std::string& fileName);

    std::map<std::string, size_t> _columns; ///< column index (without time) by name
    std::vector<double> _time;
    std::vector<std::vector<double> > _values; ///< values by column, then row
    std::vector<Binding> _bindings;
};
/** @} */ // end of dataexchange
