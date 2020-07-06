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
  // OF imports
  import Absyn;
  import AbsynUtil;
  import DAE;
  import SCode;
  import SCodeUtil;

  //NF imports
  import NFBinding.Binding;
  import NFComponent.Component;
  import ComponentRef = NFComponentRef;
  import Expression = NFExpression;
  import NFPrefixes.Direction;
  import NFInstNode.InstNode;
  import NFPrefixes.Variability;
  import Type = NFType;
  import Variable = NFVariable;

  // Util imports
  import Pointer;

public
  uniontype BackendInfo
    record BACKEND_INFO
      VariableKind varKind                  "Structural kind: state, algebraic...";
      Option<VariableAttributes> attributes "values on built-in attributes";
    end BACKEND_INFO;

    function setVarKind
      input output BackendInfo binfo;
      input VariableKind varKind;
    algorithm
      binfo.varKind := varKind;
    end setVarKind;
  end BackendInfo;

  constant BackendInfo DUMMY_BACKEND_INFO = BACKEND_INFO(FRONTEND_DUMMY(), NONE());

  uniontype VariableKind
    record ALGEBRAIC end ALGEBRAIC;
    record STATE
      Integer index                         "how often this states was differentiated";
      Option<Pointer<Variable>> derivative  "pointer to the derivative";
      Boolean natural                       "false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction";
    end STATE;
    record STATE_DER
      Pointer<Variable> state               "Original state";
      Option<Pointer<Expression>> alias     "Optional alias state expression. Result of differentiating the state if existant!";
    end STATE_DER;
    record DUMMY_DER
      Pointer<Variable> dummy_state         "corresponding dummy state";
    end DUMMY_DER;
    record DUMMY_STATE
      Pointer<Variable> dummy_der           "corresponding dummy derivative";
    end DUMMY_STATE; // ToDo: maybe dynamic state for dynamic state seleciton in index reduction
    record DISCRETE end DISCRETE;
    record DISCRETE_STATE
      Pointer<Variable> previous            "Pointer to the left limit if existant.";
      Boolean fixed                         "is fixed at first clock tick";
    end DISCRETE_STATE;
    record PREVIOUS
      Pointer<Variable> disc                "Pointer to the corresponding discrete variable.";
    end PREVIOUS;
    record PARAMETER end PARAMETER;
    record CONSTANT end CONSTANT;
    record START
      Pointer<Variable> original            "Pointer to the corresponding original variable.";
    end START;
    record EXTOBJ
      Absyn.Path fullClassName;
    end EXTOBJ;
    record JAC_VAR end JAC_VAR;
    record JAC_DIFF_VAR end JAC_DIFF_VAR;
    record SEED_VAR
      Pointer<Variable> var                 "Pointer to the variable for which the seed got created.";
    end SEED_VAR;
    record OPT_CONSTR end OPT_CONSTR;
    record OPT_FCONSTR end OPT_FCONSTR;
    record OPT_INPUT_WITH_DER end OPT_INPUT_WITH_DER;
    record OPT_INPUT_DER end OPT_INPUT_DER;
    record OPT_TGRID end OPT_TGRID;
    record OPT_LOOP_INPUT
      ComponentRef replaceCref;
    end OPT_LOOP_INPUT;
    // ToDo maybe deprecated:
    record ALG_STATE        "algebraic state used by inline solver" end ALG_STATE;
    record ALG_STATE_OLD    "algebraic state old value used by inline solver" end ALG_STATE_OLD;
    record DAE_RESIDUAL_VAR
      "variable kind used for DAEmode"
      Integer index;
    end DAE_RESIDUAL_VAR;
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
        case DISCRETE() then            "[DISC]";
        case DISCRETE_STATE() then      "[DSTA]";
        case PREVIOUS() then            "[PRE-]";
        case PARAMETER() then           "[PRMT]";
        case CONSTANT() then            "[CNST]";
        case START() then               "[STRT]";
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
        case FRONTEND_DUMMY() then      "[DMMY] Dummy Variable.";
        else "[FAIL] " + getInstanceName() + " failed.";
      end match;
    end toString;
  end VariableKind;

  uniontype VariableAttributes
    record VAR_ATTR_REAL
      Option<Expression> quantity             "quantity";
      Option<Expression> unit                 "SI Unit for actual computation value";
      Option<Expression> displayUnit          "SI Unit only for displaying";
      Option<Expression> min                  "Lower boundry";
      Option<Expression> max                  "Upper boundry";
      Option<Expression> start                "start value";
      Option<Expression> fixed                "fixed - true: default for parameter/constant, false - default for other variables";
      Option<Expression> nominal              "nominal";
      Option<StateSelect> stateSelect         "Priority to be selected as a state during index reduction";
      Option<TearingSelect> tearingSelect     "Priority to be selected as an iteration variable during tearing";
      Option<Uncertainty> uncertainty         "Attributes from data reconcilliation";
      Option<Distribution> distribution       "ToDo: ???";
      Option<Expression> binding              "A binding expression for certain types. E.G. parameters";
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
      Option<Expression> startOrigin          "where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)";
    end VAR_ATTR_REAL;

    record VAR_ATTR_INT
      Option<Expression> quantity             "quantity";
      Option<Expression> min                  "Lower boundry";
      Option<Expression> max                  "Upper boundry";
      Option<Expression> start                "start value";
      Option<Expression> fixed                "fixed - true: default for parameter/constant, false - default for other variables";
      Option<Uncertainty> uncertainty         "Attributes from data reconcilliation";
      Option<Distribution> distribution       "ToDo: ???";
      Option<Expression> binding              "A binding expression for certain types. E.G. parameters";
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
      Option<Expression> startOrigin          "where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)";
    end VAR_ATTR_INT;

    record VAR_ATTR_BOOL
      Option<Expression> quantity             "quantity";
      Option<Expression> start                "start value";
      Option<Expression> fixed                "fixed - true: default for parameter/constant, false - default for other variables";
      Option<Expression> binding              "A binding expression for certain types. E.G. parameters";
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
      Option<Expression> startOrigin          "where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)";
    end VAR_ATTR_BOOL;

    record VAR_ATTR_CLOCK
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
    end VAR_ATTR_CLOCK;

    record VAR_ATTR_STRING
      "kabdelhak: why does string have quantity/start/fixed?"
      Option<Expression> quantity             "quantity";
      Option<Expression> start                "start value";
      Option<Expression> fixed                "fixed - true: default for parameter/constant, false - default for other variables";
      Option<Expression> binding              "A binding expression for certain types. E.G. parameters";
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
      Option<Expression> startOrigin          "where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)";
    end VAR_ATTR_STRING;

    record VAR_ATTR_ENUMERATION
      Option<Expression> quantity             "quantity";
      Option<Expression> min                  "Lower boundry";
      Option<Expression> max                  "Upper boundry";
      Option<Expression> start                "start value";
      Option<Expression> fixed                "fixed - true: default for parameter/constant, false - default for other variables";
      Option<Expression> binding              "A binding expression for certain types. E.G. parameters";
      Option<Boolean> isProtected             "Defined in protected scope";
      Option<Boolean> finalPrefix             "Defined as final";
      Option<Expression> startOrigin          "where did start=X came from? NONE()|SOME(DAE.SCONST binding|type|undefined)";
    end VAR_ATTR_ENUMERATION;

    function toString
      "For usability this takes an option instead of the object itself."
      input Option<VariableAttributes> optAttributes;
      output String str;
    protected
      VariableAttributes attributes;
    algorithm
      if isSome(optAttributes) then
        SOME(attributes) := optAttributes;
        str := match attributes
          local
            VariableAttributes qual;
          case qual as VAR_ATTR_REAL()
            then "(" + attributesToString({("fixed", qual.fixed), ("start", qual.start), ("min", qual.min), ("max", qual.max), ("nominal", qual.nominal)}, qual.stateSelect, qual.tearingSelect) + ")";

          case qual as VAR_ATTR_INT()
            then "(" + attributesToString({("fixed", qual.fixed), ("start", qual.start), ("min", qual.min), ("max", qual.max)}, NONE(), NONE()) + ")";

          case qual as VAR_ATTR_BOOL()
            then "(" + attributesToString({("fixed", qual.fixed), ("start", qual.start)}, NONE(), NONE()) + ")";

          case VAR_ATTR_CLOCK()
            then "";

          case qual as VAR_ATTR_STRING()
            then "(" + attributesToString({("fixed", qual.fixed), ("start", qual.start)}, NONE(), NONE()) + ")";

          case qual as VAR_ATTR_ENUMERATION()
            then "(" + attributesToString({("fixed", qual.fixed), ("start", qual.start), ("min", qual.min), ("max", qual.max)}, NONE(), NONE()) + ")";

          else "(" + getInstanceName() + " failed. Attribute string could not be created.)";
        end match;
      else
        str := "";
      end if;
    end toString;

    function create
      input list<tuple<String, Binding>> attrs;
      input Type ty;
      input Component.Attributes compAttrs;
      input Option<SCode.Comment> comment;
      output Option<VariableAttributes> attributes;
    protected
      Boolean is_final;
      Option<Boolean> is_final_opt;
      Type elTy;
      Boolean is_array = false;
    algorithm
      is_final := compAttrs.isFinal or
                  compAttrs.variability == Variability.STRUCTURAL_PARAMETER;

      if listEmpty(attrs) and not is_final then
        // kabdelhak: i think we should create a default one here and do not have it as an option
        // more robust in the backend than always checking for NONE() and having to interprete it
        attributes := NONE();
        return;
      end if;

      is_final_opt := SOME(is_final);

      attributes := match Type.arrayElementType(ty)
        case Type.REAL() then createReal(attrs, is_final_opt, comment);
        case Type.INTEGER() then createInt(attrs, is_final_opt);
        case Type.BOOLEAN() then createBool(attrs, is_final_opt);
        case Type.STRING() then createString(attrs, is_final_opt);
        case Type.ENUMERATION() then createEnum(attrs, is_final_opt);
        else NONE();
      end match;
    end create;

  protected
    function attributesToString
      input list<tuple<String, Option<Expression>>> tpl_lst;
      input Option<StateSelect> stateSelect;
      input Option<TearingSelect> tearingSelect;
      output String str = "";
    protected
      list<String> buffer = {};
      String name;
    algorithm
      for tpl in tpl_lst loop
        buffer := attributeToString(tpl, buffer);
      end for;

      buffer := stateSelectString(stateSelect, buffer);
      buffer := tearingSelectString(tearingSelect, buffer);

      buffer := listReverse(buffer);

      if not listEmpty(buffer) then
        name :: buffer := buffer;
        str := str + name;
        for name in buffer loop
          str := str + ", " + name;
        end for;
      end if;
    end attributesToString;

    function attributeToString
      "Creates an optional string for an optional attribute."
      input tuple<String, Option<Expression>> tpl;
      input output list<String> buffer;
    protected
      String name;
      Option<Expression> optAttr;
      Expression attr;
    algorithm
      (name, optAttr) := tpl;
      if isSome(optAttr) then
        SOME(attr) := optAttr;
        buffer := name + " = " + Expression.toString(attr) :: buffer;
      end if;
    end attributeToString;

    function stateSelectString
      input Option<StateSelect> optStateSelect;
      input output list<String> buffer;
    protected
      StateSelect stateSelect;
    algorithm
      if isSome(optStateSelect) then
        SOME(stateSelect) := optStateSelect;
        buffer := match stateSelect
          case StateSelect.NEVER then "StateSelect = never" :: buffer;
          case StateSelect.AVOID then "StateSelect = avoid" :: buffer;
          case StateSelect.DEFAULT then "StateSelect = default" :: buffer;
          case StateSelect.PREFER then "StateSelect = prefer" :: buffer;
          case StateSelect.ALWAYS then "StateSelect = always" :: buffer;
        end match;
      end if;
    end stateSelectString;

    function tearingSelectString
      input Option<TearingSelect> optTearingSelect;
      input output list<String> buffer;
    protected
      TearingSelect tearingSelect;
    algorithm
      if isSome(optTearingSelect) then
        SOME(tearingSelect) := optTearingSelect;
        buffer := match tearingSelect
          case TearingSelect.NEVER then "TearingSelect = never" :: buffer;
          case TearingSelect.AVOID then "TearingSelect = avoid" :: buffer;
          case TearingSelect.DEFAULT then "TearingSelect = default" :: buffer;
          case TearingSelect.PREFER then "TearingSelect = prefer" :: buffer;
          case TearingSelect.ALWAYS then "TearingSelect = always" :: buffer;
        end match;
      end if;
    end tearingSelectString;

    function createReal
      input list<tuple<String, Binding>> attrs;
      input Option<Boolean> isFinal;
      input Option<SCode.Comment> comment;
      output Option<VariableAttributes> attributes;
    protected
      String name;
      Binding b;
      Option<Expression> quantity = NONE(), unit = NONE(), displayUnit = NONE();
      Option<Expression> min = NONE(), max = NONE(), start = NONE(), fixed = NONE(), nominal = NONE();
      Option<StateSelect> state_select = NONE();
      Option<TearingSelect> tearing_select = NONE();
    algorithm
      for attr in attrs loop
        (name, b) := attr;
        () := match name
          case "displayUnit"    algorithm displayUnit := createAttribute(b); then ();
          case "fixed"          algorithm fixed := createAttribute(b); then ();
          case "max"            algorithm max := createAttribute(b); then ();
          case "min"            algorithm min := createAttribute(b); then ();
          case "nominal"        algorithm nominal := createAttribute(b); then ();
          case "quantity"       algorithm quantity := createAttribute(b); then ();
          case "start"          algorithm start := createAttribute(b); then ();
          case "stateSelect"    algorithm state_select := createStateSelect(b); then ();
          // TODO: VAR_ATTR_REAL has no field for unbounded.
          case "unbounded"      then ();
          case "unit"           algorithm unit := createAttribute(b); then ();

          // The attributes should already be type checked, so we shouldn't get any
          // unknown attributes here.
          else
            algorithm
              Error.assertion(false, getInstanceName() + " got unknown type attribute " + name, sourceInfo());
            then
              fail();
        end match;
      end for;
      tearing_select := createTearingSelect(comment);

      attributes := SOME(VariableAttributes.VAR_ATTR_REAL(
        quantity, unit, displayUnit, min, max, start, fixed, nominal,
        state_select, tearing_select, NONE(), NONE(), NONE(), NONE(), isFinal, NONE()));
    end createReal;

    function createInt
      input list<tuple<String, Binding>> attrs;
      input Option<Boolean> isFinal;
      output Option<VariableAttributes> attributes;
    protected
      String name;
      Binding b;
      Option<Expression> quantity = NONE(), min = NONE(), max = NONE();
      Option<Expression> start = NONE(), fixed = NONE();
    algorithm
      for attr in attrs loop
        (name, b) := attr;

        () := match name
          case "quantity" algorithm quantity := createAttribute(b); then ();
          case "min"      algorithm min := createAttribute(b); then ();
          case "max"      algorithm max := createAttribute(b); then ();
          case "start"    algorithm start := createAttribute(b); then ();
          case "fixed"    algorithm fixed := createAttribute(b); then ();

          // The attributes should already be type checked, so we shouldn't get any
          // unknown attributes here.
          else
            algorithm
              Error.assertion(false, getInstanceName() + " got unknown type attribute " + name, sourceInfo());
            then
              fail();
        end match;
      end for;

      attributes := SOME(VariableAttributes.VAR_ATTR_INT(
        quantity, min, max, start, fixed,
        NONE(), NONE(), NONE(), NONE(), isFinal, NONE()));
    end createInt;

    function createBool
      input list<tuple<String, Binding>> attrs;
      input Option<Boolean> isFinal;
      output Option<VariableAttributes> attributes;
    protected
      String name;
      Binding b;
      Option<Expression> quantity = NONE(), start = NONE(), fixed = NONE();
    algorithm
      for attr in attrs loop
        (name, b) := attr;

        () := match name
          case "quantity" algorithm quantity := createAttribute(b); then ();
          case "start"    algorithm start := createAttribute(b); then ();
          case "fixed"    algorithm fixed := createAttribute(b); then ();

          // The attributes should already be type checked, so we shouldn't get any
          // unknown attributes here.
          else
            algorithm
              Error.assertion(false, getInstanceName() + " got unknown type attribute " + name, sourceInfo());
            then
              fail();
        end match;
      end for;

      attributes := SOME(VariableAttributes.VAR_ATTR_BOOL(
        quantity, start, fixed, NONE(), NONE(), isFinal, NONE()));
    end createBool;

    function createString
      input list<tuple<String, Binding>> attrs;
      input Option<Boolean> isFinal;
      output Option<VariableAttributes> attributes;
    protected
      String name;
      Binding b;
      Option<Expression> quantity = NONE(), start = NONE(), fixed = NONE();
    algorithm
      for attr in attrs loop
        (name, b) := attr;

        () := match name
          case "quantity" algorithm quantity := createAttribute(b); then ();
          case "start"    algorithm start := createAttribute(b); then ();
          case "fixed"    algorithm fixed := createAttribute(b); then ();

          // The attributes should already be type checked, so we shouldn't get any
          // unknown attributes here.
          else
            algorithm
              Error.assertion(false, getInstanceName() + " got unknown type attribute " + name, sourceInfo());
            then
              fail();
        end match;
      end for;

      attributes := SOME(VariableAttributes.VAR_ATTR_STRING(
        quantity, start, fixed, NONE(), NONE(), isFinal, NONE()));
    end createString;

    function createEnum
      input list<tuple<String, Binding>> attrs;
      input Option<Boolean> isFinal;
      output Option<VariableAttributes> attributes;
    protected
      String name;
      Binding b;
      Option<Expression> quantity = NONE(), min = NONE(), max = NONE();
      Option<Expression> start = NONE(), fixed = NONE();
    algorithm
      for attr in attrs loop
        (name, b) := attr;

        () := match name
          case "fixed"       algorithm fixed := createAttribute(b); then ();
          case "max"         algorithm max := createAttribute(b); then ();
          case "min"         algorithm min := createAttribute(b); then ();
          case "quantity"    algorithm quantity := createAttribute(b); then ();
          case "start"       algorithm start := createAttribute(b); then ();

          // The attributes should already be type checked, so we shouldn't get any
          // unknown attributes here.
          else
            algorithm
              Error.assertion(false, getInstanceName() + " got unknown type attribute " + name, sourceInfo());
            then
              fail();
        end match;
      end for;

      attributes := SOME(VariableAttributes.VAR_ATTR_ENUMERATION(
        quantity, min, max, start, fixed, NONE(), NONE(), isFinal, NONE()));
    end createEnum;

    function createAttribute
      input Binding binding;
      output Option<Expression> attribute = SOME(Binding.getTypedExp(binding));
    end createAttribute;

    function createStateSelect
      input Binding binding;
      output Option<StateSelect> stateSelect;
    protected
      InstNode node;
      String name;
      Expression exp = Expression.getBindingExp(Binding.getTypedExp(binding));
    algorithm
      name := match exp
        case Expression.ENUM_LITERAL() then exp.name;
        case Expression.CREF(cref = ComponentRef.CREF(node = node)) then InstNode.name(node);
        else
          algorithm
            Error.assertion(false, getInstanceName() +
              " got invalid StateSelect expression " + Expression.toString(exp), sourceInfo());
          then
            fail();
      end match;

      stateSelect := SOME(lookupStateSelectMember(name));
    end createStateSelect;

    function createTearingSelect
      "tearingSelect is an annotation and has to be extracted from the comment."
      input Option<SCode.Comment> optComment;
      output Option<TearingSelect> tearingSelect;
    protected
      SCode.Annotation anno;
      Absyn.Exp val;
      String name;
    algorithm
      try
        SOME(SCode.COMMENT(annotation_=SOME(anno))) := optComment;
        val := SCodeUtil.getNamedAnnotation(anno, "tearingSelect");
        name := AbsynUtil.crefIdent(AbsynUtil.expCref(val));
        tearingSelect := SOME(lookupTearingSelectMember(name));
      else
        tearingSelect := NONE();
      end try;
    end createTearingSelect;

    function lookupStateSelectMember
      input String name;
      output StateSelect stateSelect;
    algorithm
      stateSelect := match name
        case "never" then StateSelect.NEVER;
        case "avoid" then StateSelect.AVOID;
        case "default" then StateSelect.DEFAULT;
        case "prefer" then StateSelect.PREFER;
        case "always" then StateSelect.ALWAYS;
        else
          algorithm
            Error.assertion(false, getInstanceName() + " got unknown StateSelect literal " + name, sourceInfo());
          then
            fail();
      end match;
    end lookupStateSelectMember;

    function lookupTearingSelectMember
      input String name;
      output StateSelect stateSelect;
    algorithm
      stateSelect := match name
        case "never" then TearingSelect.NEVER;
        case "avoid" then TearingSelect.AVOID;
        case "default" then TearingSelect.DEFAULT;
        case "prefer" then TearingSelect.PREFER;
        case "always" then TearingSelect.ALWAYS;
        else
          algorithm
            Error.assertion(false, getInstanceName() + " got unknown TearingSelect literal " + name, sourceInfo());
          then
            fail();
      end match;
    end lookupTearingSelectMember;

  end VariableAttributes;

  constant VariableAttributes emptyVarAttrReal = VAR_ATTR_REAL(NONE(),NONE(),NONE(), NONE(), NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE());
  constant VariableAttributes emptyVarAttrBool = VAR_ATTR_BOOL(NONE(),NONE(),NONE(),NONE(),NONE(),NONE(),NONE());

  type StateSelect = enumeration(NEVER, AVOID, DEFAULT, PREFER, ALWAYS);
  type TearingSelect = enumeration(NEVER, AVOID, DEFAULT, PREFER, ALWAYS);
  type Uncertainty = enumeration(GIVEN, SOUGHT, REFINE);

  uniontype Distribution
    record DISTRIBUTION
      Expression name;
      Expression params;
      Expression paramNames;
    end DISTRIBUTION;
  end Distribution;

    annotation(__OpenModelica_Interface="frontend");
end NFBackendExtension;
