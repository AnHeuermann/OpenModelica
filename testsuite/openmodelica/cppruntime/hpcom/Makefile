TEST = ../../../rtest -v

TESTFILES = \
Modelica.Electrical.Analog.Examples.CauerLowPassSC_levelfix_pthreads_memory.mos \
Modelica.Electrical.Spice3.Examples.CoupledInductors_list_pthreads_spin.mos

TESTFILES_ALL = $(TESTFILES_SERIAL) $(TESTFILES_LEVELFIX) $(TESTFILES_LEVEL) $(TESTFILES_METIS) $(TESTFILES_LIST) $(TESTFILES_LISTR) $(TESTFILES_TBB) $(TESTFILES_MCP)

TESTFILES_SERIAL =

TESTFILES_LEVELFIX = \
Modelica.Electrical.Analog.Examples.CauerLowPassSC_levelfix_pthreads_memory.mos

TESTFILES_LEVEL = \

TESTFILES_METIS =

TESTFILES_LIST = \
Modelica.Electrical.Spice3.Examples.CoupledInductors_list_pthreads_spin.mos

TESTFILES_LISTR =

TESTFILES_TBB =

TESTFILES_MCP =

# Run make failingtest
FAILINGTESTFILES= \
Modelica.Mechanics.MultiBody.Examples.Elementary.ThreeSprings_level_omp.mos \

# Dependency files that are not .mo .mos or Makefile
# Add them here or they will be cleaned.
DEPENDENCIES = \
*.mo \
*.mos \
Makefile \
ReferenceFiles \
ReferenceGraphs \
fmu


CLEAN = `ls | grep -w -v -f deps.tmp`

.PHONY : test testAll clean getdeps failingtest testSerial testLevelfix testMetis testList testListr testTbb testMcp

test:
	@echo
	@echo Running tests...
	@echo
	@echo OPENMODELICAHOME=" $(OPENMODELICAHOME) "
	@$(TEST) $(TESTFILES)

testSerial:
	@$(TEST) -disabled $(TESTFILES_SERIAL)

testLevelfix:
	@$(TEST) -disabled $(TESTFILES_LEVELFIX)

testLevel:
	@$(TEST) -disabled $(TESTFILES_LEVEL)

testMetis:
	@$(TEST) -disabled $(TESTFILES_METIS)

testList:
	@$(TEST) -disabled $(TESTFILES_LIST)

testListr:
	@$(TEST) -disabled $(TESTFILES_LISTR)

testTbb:
	@$(TEST) -disabled $(TESTFILES_TBB)

testMcp:
	@$(TEST) -disabled $(TESTFILES_MCP)

testAll:
	@echo
	@echo Running all tests...
	@echo
	@echo OPENMODELICAHOME=" $(OPENMODELICAHOME) "
	@$(TEST) -disabled $(TESTFILES_ALL)

# Cleans all files that are not listed as dependencies
clean :
	@echo $(DEPENDENCIES) | sed 's/ /\\|/g' > deps.tmp
	@rm -f $(CLEAN)

# Run this if you want to list out the files (dependencies).
# do it after cleaning and updating the folder
# then you can get a list of file names (which must be dependencies
# since you got them from repository + your own new files)
# then add them to the DEPENDENCIES. You can find the
# list in deps.txt
getdeps:
	@echo $(DEPENDENCIES) | sed 's/ /\\|/g' > deps.tmp
	@echo $(CLEAN) | sed -r 's/deps.txt|deps.tmp//g' | sed 's/ / \\\n/g' > deps.txt
	@echo Dependency list saved in deps.txt.
	@echo Copy the list from deps.txt and add it to the Makefile @DEPENDENCIES

failingtest:
	@echo
	@echo Running failing tests...
	@echo
	@$(TEST) -disabled $(FAILINGTESTFILES)
