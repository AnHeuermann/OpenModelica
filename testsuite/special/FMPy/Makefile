
FMPY_CALL = python3 -m fmpy
FMU_EXPORT_DIR ?= CMakeCrossCompile

FMU_DIRECTORY = ../FmuExportCrossCompile/$(FMU_EXPORT_DIR)
FMU = FmuExportCrossCompile.fmu \
      RoomHeating_OM_RH.fmu \
      WaterTank_Control.fmu \
      WaterTank_TestSingleWaterTank.fmu \
      BouncingBall.fmu

.PHONY: all test

all: compile_FMUs
	$(MAKE) test FMU_EXPORT_DIR=MakeCrossCompile
	$(MAKE) test FMU_EXPORT_DIR=CMakeCrossCompile
test: $(FMU)

%.fmu:
	@echo -----------------------------------------------------------------------
	@echo $@
  # Remove binaries
	unzip -qq $(FMU_DIRECTORY)/$@ -d $@_FMU/
	rm -rf $@_FMU/binaries
	cd $@_FMU/; zip -qqr ../$@ *
	rm -rf $@_FMU/
  # Compile sources and simulate
	$(FMPY_CALL) compile $@
	$(FMPY_CALL) simulate $@
	@echo -----------------------------------------------------------------------

compile_FMUs:
	make -C ../FmuExportCrossCompile

clean:
	rm -f $(FMU)
	rm -rf $(addsuffix _FMU/,$(FMU))
