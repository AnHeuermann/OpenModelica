spackage test
  
  package builtin
	function testfn
	  input String inString;
	  output Integer outString;
	end testfn;
	
	function listLength "Return the length of the list"
      replaceable type TypeVar subtypeof Any;    
      input list<TypeVar> lst;
      output Integer result;
    end listLength;
    
    function listMember "Verify if an element is part of the list"
      replaceable type TypeVar subtypeof Any;
      input TypeVar element;
      input list<TypeVar> lst;
      output Boolean result;
    end listMember;
	
	function listGet "Return the element of the list at the given index.
                      The index starts from 1."
      input list<TypeVar> lst;
      input Integer index;
      output TypeVar result;
      replaceable type TypeVar subtypeof Any;
    end listGet;
	
	function listReverse "Reverse the order of elements in the list"
      replaceable type TypeVar subtypeof Any;
      input list<TypeVar> lst;
      output list<TypeVar> result;
    end listReverse;
	
  end builtin;
  
  package TplAbsyn
    type Ident = String;
  	type TypedIdents = list<tuple<Ident, PathIdent>>;
	
	  uniontype PathIdent
	    record IDENT
	      Ident ident;    
	    end IDENT;
	  
	    record PATH_IDENT
	      Ident ident;
	      PathIdent path;
	    end PATH_IDENT;
	  end PathIdent;
  end TplAbsyn;


pathIdent(PathIdent) ::= 
  case IDENT      then ident
  case PATH_IDENT then '<ident>.<pathIdent(path)>'
	
typedIdents(TypedIdents decls) ::= 
(decls of (id,pid) : 
   '<pathIdent(pid)> <id>;//heja' 
   \n 
)

test(list<String> items, Integer ind) ::= (items ind; align=testfn(ind); alignSeparator='ss<ind>'; wrapSeparator=testfn(2))

test2(list<String> items, String sep, Integer a) ::= (items sep; align=a)

test3(list<String> items, String item, Integer ii) ::= 
  <<
  <[items, item, ii] of st: 'bla<st>' \n>
  <[items, item, ii, ([items, item, ii]\n), "blaaa" ] ", ">
  <[items, item, ii] ", "/*]*/>!!!!!error should be
  <[items, item, ii, ([items, item, ii]\n), "blaaa" ] : case it then it ", ">
  <match 'aha<ii>' case it then it>
>>

testCond(Option<tuple<String,Integer>> nvOpt) ::= 
  if nvOpt is SOME((name,value)) then '<name> = <value>;'
  else "no value"

testCond2(Option<tuple<String,Integer>> nvOpt) ::= 
  if nvOpt is not SOME((name,value)) then "none" 
  else 'SOME(<name>,<value>)' 

mapInt(Integer) ::= '(int:<it>)'
mapString(String) ::= '(str:<it>)'
mapIntString(Integer intPar, String stPar) ::= '(int:<intPar>,str:<stPar>)'

testMap(list<Integer> ints) ::= (ints : mapInt() : mapString() ", ")
testMap2(list<Integer> ints) ::= (ints of int : mapInt() of st : mapIntString(int, st) ", ")
testMap3(list<list<Integer>> lstOfLst) ::= 
	(lstOfLst of intLst : 
		(intLst of int : mapInt(int) ", ") 
	";\n"; anchor)
testMap4(list<list<Integer>> lstOfLst) ::= lstOfLst : it : mapInt()
testMap5(list<Integer> ints) ::= (ints : mapString(mapInt()) ", ")

intMatrix(list<list<Integer>> lstOfLst) ::= 
<< 
[ <lstOfLst of intLst : 
		(intLst ", ") 
   ";\n"; anchor> ]
>>

ifTest(Integer i) ::= if mapInt(i) then '<it> name;' else "/* weird I */"

bindTest() ::= 
  ifTest(1) of ii : 
    <<
      some hej<ii>
    >>

txtTest() ::= 
  # txt = "ahoj"
  # txt += "hej"
  txt

txtTest2() ::= 
<<
<# txt = "ahoj2" #>
<# txt += "hej2" #>
bl�� <txt>
  </* jhgjhgjh  */>  
jo
>>

txtTest3(String hej, Text buf) ::= 
<<
<# txt = "aahoj2" #>
<# txt += "ahej2" #>
<# buf += txt #>
<# buf += '<txtTest4("ha!",buf)>ahoj' //TODO: not allow this ...  
#>
abl�� <txt>
  </* jhgjhgjh  */>  
ajo
>>

txtTest4(String hej, Text buf) ::= 
if hej then 
  # txt = "ahoj2"
  # txt += hej
  # buf += txt
  <<
  bl�� <txt>
  </* jhgjhgjh  */>  
  jo
  >>

txtTest5(String hej, Text buf, Text nobuf) ::= 
<<
<# txt = "aahoj2" #>
<# txt += "ahej2" #>
<# buf += txt #>
<# buf += '<txtTest4("ha!",buf)>ahoj' //TODO: not allow this ...  
#>
abl�� <txt>
  </* jhgjhgjh  */>  
ajo
>>

txtTest6(list<String> hej, Text buf) ::=
  # mytxt = "bolo"
  # nomut = ','
  
  case "1"::_ then
    # buf2 = "hop"
    (hej : 
      # buf2 += it 
      # mytxt += '<it>jo'
      '<it><nomut>'
     nomut)
  
  case h::_ then
    # buf2 = "hop"
    (h : 
      # buf2 += it 
      # mytxt += '<it>jo'
      '<it><nomut>'
     nomut)

contCase(String tst) ::=
  case "a"
  case "b"
  case "bb"
  case "c" then "hej"
  case "d" then "Hej!"

contCase2(PathIdent) ::=
  case IDENT
  case PATH_IDENT 
  case IDENT(ident = "ii")
    then 'id=<ident>'
  case IDENT then "hej"


/*
  case skdflk then
    <<
    something
    >>
  case sdjfk then <<
    something
  >>
  
  if sdklfn then 
    <<
    bla something
    dfgf
    >> 
  else 
    <<
    bla else something
    sdf
    >>
 */

genericTest(list<String> lst) ::= listLength(lst)  
genericTest2(list<Integer> lst) ::= listLength(lst)
genericTest3(list<Integer> lst) ::= listMember(3,lst)
genericTest4(list<String> lst) ::= listMember("ahoj",lst)
genericTest5(list<String> lst, String hoj) ::= listMember('a<hoj>',lst)
genericTest6(list<String> lst, Integer idx) ::= listGet(lst,idx)
genericTest7(list<Integer> lst, Integer idx) ::= listGet(lst,idx)
genericTest8(list<Integer> lst) ::= listReverse(lst) : '<it>th revesed'
genericTest9(list<list<String>> lst) ::= listReverse() : listReverse() : '<it>hej!'

//Error - unmatched type for type variable 'TypeVar'. Firstly inferred 'String', next inferred 'Integer'(dealiased 'Integer').
//genericTest10(list<Integer> lst) ::= listMember("3",lst) 


end test;
