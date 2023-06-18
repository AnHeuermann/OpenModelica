/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3,
 * ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from OSMC, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or
 * http://www.openmodelica.org, and in the OpenModelica distribution.
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
/*
 * @author Volker Waurich <volker.waurich@tu-dresden.de>
 */

#include "FMUWrapper.h"
#include "Modeling/MessagesWidget.h"
#include "Util/Helper.h"
#include "FMU2Wrapper.h"

SimSettingsFMU::SimSettingsFMU()
                : _callEventUpdate(fmi2_false),
                  _toleranceControlled(fmi2_true),
                  _intermediateResults(fmi2_false),
                  _tstart(0.0),
                  _hdef(0.1),
                  _tend(0.1),
                  _relativeTolerance(0.001),
                  _solver(Solver::EULER_FORWARD),
                  mIterateEvents(true)
{
}

void SimSettingsFMU::setTend(const double t)
{
  _tend = t;
}

void SimSettingsFMU::setTstart(const double t)
{
  _tstart = t;
}

void SimSettingsFMU::setHdef(const double h)
{
  _hdef = h;
}

void SimSettingsFMU::setRelativeTolerance(const double t)
{
  _relativeTolerance = t;
}

double SimSettingsFMU::getTend() const
{
  return _tend;
}

double SimSettingsFMU::getTstart() const
{
  return _tstart;
}

double SimSettingsFMU::getHdef()
{
  return _hdef;
}

double SimSettingsFMU::getRelativeTolerance()
{
  return _relativeTolerance;
}

int SimSettingsFMU::getToleranceControlled() const
{
  return _toleranceControlled;
}

void SimSettingsFMU::setSolver(const Solver& solver)
{
  _solver = solver;
}

int* SimSettingsFMU::getCallEventUpdate()
{
  return &_callEventUpdate;
}

int SimSettingsFMU::getIntermediateResults()
{
  return _intermediateResults;
}

void SimSettingsFMU::setIterateEvents(bool iE)
{
  mIterateEvents = iE;
}

bool SimSettingsFMU::getIterateEvents()
{
  return mIterateEvents;
}
//-------------------------------
// Abstract FMU class
//-------------------------------


FMUWrapperAbstract::FMUWrapperAbstract(){
}
