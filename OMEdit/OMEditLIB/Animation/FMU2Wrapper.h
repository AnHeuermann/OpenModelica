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

#ifndef __FMU_2_WRAPPER_H
#define __FMU_2_WRAPPER_H

class FMUWrapper_ME_2 : public FMUWrapperAbstract
{
 public:

  FMUWrapper_ME_2();
  ~FMUWrapper_ME_2();

  void load(const std::string& modelFile, const std::string& path, fmi_import_context_t* mpContext);
  void initialize(const std::shared_ptr<SimSettingsFMU> simSettings);
  void setContinuousStates();
  bool checkForTriggeredEvent();
  bool itsEventTime();
  void updateNextTimeStep(const double hdef);
  void handleEvents(const int intermediateResults);
  void prepareSimulationStep(const double time);
  void setLastStepSize(const double simTimeEnd);
  void solveSystem();
  void doEulerStep();
  void completedIntegratorStep(int* callEventUpdate);
  void do_event_iteration(fmi2_import_t *fmu, fmi2_event_info_t *eventInfo);

  const FMUData* getFMUData();
  fmi2_import_t* getFMU();
  void fmi_get_real(unsigned int* valueRef, double* res);
  unsigned int fmi_get_variable_by_name(const char* name);

 private:
  fmi2_import_t* mpFMU;
  fmi2_callback_functions_t mCallBackFunctions;
  FMUData mFMUdata;
};

#endif // end __FMU_2_WRAPPER_H
