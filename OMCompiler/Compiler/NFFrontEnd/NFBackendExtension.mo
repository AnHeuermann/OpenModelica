/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2020, Linköping University,
 * Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3
 * AND THIS OSMC PUBLIC LICENSE (OSMC-PL).
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S
 * ACCEPTANCE OF THE OSMC PUBLIC LICENSE.
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from Linköping University, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or
 * http://www.openmodelica.org, and in the OpenModelica distribution.
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS
 * OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
encapsulated package NFBackendExtension
  " ==========================================================================
    kabdelhak: The following structures are used only in the backend to avoid a
    forward dependency of the frontend to the backend. All functions for these
    structures are defined in NBVariable.mo
  ========================================================================== "

protected
  import Absyn;
  import ComponentRef = NFComponentRef;
  import DAE;

public
  uniontype BackendInfo
    record BACKEND_INFO
      VariableKind varKind                          "Structural kind: state, algebraic...";
      // Merge Tearing Select to VariableAttributes?
      Option<DAE.VariableAttributes> attributes     "values on built-in attributes";
      Option<TearingSelect> tearingSelect           "value for TearingSelect";
    end BACKEND_INFO;
  end BackendInfo;

  constant BackendInfo DUMMY_BACKEND_INFO = BACKEND_INFO(FRONTEND_DUMMY(), NONE(), NONE());

  uniontype VariableKind
    record ALGEBRAIC end ALGEBRAIC;
    record STATE
      Integer index                 "how often this states was differentiated";
      Option<ComponentRef> derName  "the name of the derivative";
      Boolean natural               "false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction";
    end STATE;
    record STATE_DER
      ComponentRef state            "Original state name";
    end STATE_DER;
    record DUMMY_DER end DUMMY_DER;
    record DUMMY_STATE end DUMMY_STATE;
    record CLOCKED_STATE
      ComponentRef previousName     "the name of the previous variable";
      Boolean isStartFixed          "is fixed at first clock tick";
    end CLOCKED_STATE;
    record DISCRETE end DISCRETE;
    record PREVIOUS end PREVIOUS;
    record PARAMETER end PARAMETER;
    record CONSTANT end CONSTANT;
    record EXTOBJ
      Absyn.Path fullClassName;
    end EXTOBJ;
    record JAC_VAR end JAC_VAR;
    record JAC_DIFF_VAR end JAC_DIFF_VAR;
    record SEED_VAR end SEED_VAR;
    record OPT_CONSTR end OPT_CONSTR;
    record OPT_FCONSTR end OPT_FCONSTR;
    record OPT_INPUT_WITH_DER end OPT_INPUT_WITH_DER;
    record OPT_INPUT_DER end OPT_INPUT_DER;
    record OPT_TGRID end OPT_TGRID;
    record OPT_LOOP_INPUT
      ComponentRef replaceCref;
    end OPT_LOOP_INPUT;
    record ALG_STATE        "algebraic state used by inline solver" end ALG_STATE;
    record ALG_STATE_OLD    "algebraic state old value used by inline solver" end ALG_STATE_OLD;
    record DAE_RESIDUAL_VAR "variable kind used for DAEmode" end DAE_RESIDUAL_VAR;
    record DAE_AUX_VAR      "auxiliary variable used for DAEmode" end DAE_AUX_VAR;
    record LOOP_ITERATION   "used in SIMCODE, iteration variables in algebraic loops" end LOOP_ITERATION;
    record LOOP_SOLVED      "used in SIMCODE, inner variables of a torn algebraic loop" end LOOP_SOLVED;
    record FRONTEND_DUMMY   "Undefined variable type. Only to be used during frontend phase." end FRONTEND_DUMMY;

    function toString
      input VariableKind varKind;
      output String str;
    algorithm
      str := match varKind
        case ALGEBRAIC() then           "[ALGB]";
        case STATE() then               "[STAT]";
        case STATE_DER() then           "[DER-]";
        case DUMMY_DER() then           "[DDER]";
        case DUMMY_STATE() then         "[DSTA]";
        case CLOCKED_STATE() then       "[CLCK]";
        case DISCRETE() then            "[DISC]";
        case PREVIOUS() then            "[PREV]";
        case PARAMETER() then           "[PRMT]";
        case CONSTANT() then            "[CNST]";
        case EXTOBJ() then              "[EXTO]";
        case JAC_VAR() then             "[JACV]";
        case JAC_DIFF_VAR() then        "[JACD]";
        case SEED_VAR() then            "[SEED]";
        case OPT_CONSTR() then          "[OPT][CONS]";
        case OPT_FCONSTR() then         "[OPT][FCON]]";
        case OPT_INPUT_WITH_DER() then  "[OPT][INWD]";
        case OPT_INPUT_DER() then       "[OPT][INPD]";
        case OPT_TGRID() then           "[OPT][TGRD]";
        case OPT_LOOP_INPUT() then      "[OPT][LOOP]";
        case ALG_STATE() then           "[ASTA]";
        case DAE_RESIDUAL_VAR() then    "[RES-]";
        case DAE_AUX_VAR() then         "[AUX-]";
        case LOOP_ITERATION() then      "[LOOP]";
        case LOOP_SOLVED() then         "[INNR]";
        case FRONTEND_DUMMY() then      "[DUMY] Dummy Variable.";
        else "[FAIL] NFBackendExtension.VariableKind.toString failed.";
      end match;
    end toString;
  end VariableKind;

  uniontype TearingSelect
    record NEVER end NEVER;
    record AVOID end AVOID;
    record DEFAULT end DEFAULT;
    record PREFER end PREFER;
    record ALWAYS end ALWAYS;
  end TearingSelect;

    annotation(__OpenModelica_Interface="frontend");
end NFBackendExtension;
