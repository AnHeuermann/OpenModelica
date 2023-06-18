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

#ifndef FMUWRAPPER_H
#define FMUWRAPPER_H

#include "AnimationUtil.h"

#include "fmilib.h"
#include <iostream>
#include <memory>
#include <map>

typedef struct
{
  double* _states;
  double* _statesDer;
  double* _eventIndicators;
  double* _eventIndicatorsPrev;
  unsigned int* _stateVRs;
  unsigned int* _inputVRs;
  unsigned int* _paramVRs;
  std::vector<std::string> _stateNames;
  size_t _nStates;
  size_t _nEventIndicators;
  fmi2_status_t fmiStatus2;
  fmi2_event_info_t eventInfo2;
  double _tcur;
  double _hcur;
  fmi2_boolean_t terminateSimulation;
} FMUData;


enum class Solver
{
  NONE = 0,
  EULER_FORWARD = 1
};

class SimSettingsFMU
{
 public:
  SimSettingsFMU();
  ~SimSettingsFMU() = default;
  SimSettingsFMU(const SimSettingsFMU& ss) = delete;
  SimSettingsFMU& operator=(const SimSettingsFMU& ss) = delete;
  void setTend(const fmi2_real_t t);
  double getTend() const;
  void setTstart(const fmi2_real_t t);
  double getTstart() const;
  void setHdef(const fmi2_real_t h);
  double getHdef();
  void setRelativeTolerance(const fmi2_real_t t);
  double getRelativeTolerance();
  int getToleranceControlled() const;
  void setSolver(const Solver& solver);
  int* getCallEventUpdate();
  int getIntermediateResults();
  void setIterateEvents(bool iE);
  bool getIterateEvents();

 private:
  int _callEventUpdate;
  int _toleranceControlled;
  int _intermediateResults;
  double _tstart;
  double _hdef;
  double _tend;
  double _relativeTolerance;
  Solver _solver;
  bool mIterateEvents;
};


class FMUWrapperAbstract
{
 public:
  FMUWrapperAbstract();
  virtual ~FMUWrapperAbstract() = default;

  virtual void load(const std::string& modelFile, const std::string& path, fmi_import_context_t* mpContext) = 0;
  virtual void initialize(const std::shared_ptr<SimSettingsFMU> simSettings) = 0;
  //to run simulation
  virtual bool checkForTriggeredEvent() = 0;
  virtual bool itsEventTime() = 0;
  virtual void handleEvents(const int intermediateResults) = 0;
  virtual void prepareSimulationStep(const double time) = 0;
  virtual void updateNextTimeStep(const double hdef) = 0;
  virtual void setLastStepSize(const double simTimeEnd) = 0;
  virtual void solveSystem() = 0;
  virtual void doEulerStep() = 0;
  virtual void setContinuousStates() = 0;
  virtual void completedIntegratorStep(int* callEventUpdate) = 0;

  virtual const FMUData* getFMUData()  = 0;
  virtual void fmi_get_real(unsigned int* valueRef, double* res) = 0;
  virtual unsigned int fmi_get_variable_by_name(const char* name) = 0;
};

#endif // end FMUWRAPPER_H
