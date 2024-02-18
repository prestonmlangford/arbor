PRESET := cfg/preset.cfg
CFGS := $(notdir $(basename $(wildcard cfg/*.mk)))

config-list: 
	@$(info config options:)
	@$(foreach c, $(CFGS), $(info  $c))
	@:

config:
	@echo $(preset) > $(PRESET)

ifdef cfg
CFG = $(cfg)
else
CFG := $(shell test -e $(PRESET) && cat $(PRESET))
endif

ifeq ($(filter $(CFG),$(CFGS)),)
# $(warning No configuration specified)
else

include cfg/$(CFG).mk

ROOT 			:= $(PWD)
SRC 			:= $(ROOT)/src
BLD 			:= $(ROOT)/build
BIN				:= $(BLD)/$(CFG)/bin
OBJ 			:= $(BLD)/$(CFG)/obj

ARBOR 			:= $(SRC)/arbor
UTIL 			:= $(SRC)/util
CLI 			:= $(SRC)/cli

GAMES 			:= $(SRC)/games
REVERSI 		:= $(GAMES)/reversi
BAD_BATTLESHIP 	:= $(GAMES)/bad_battleship
DICE 			:= $(GAMES)/dice
RPS 			:= $(GAMES)/rps

UT 				:= $(SRC)/ut
UT_REVERSI 		:= $(UT)/reversi

# removes duplicates in a list
set=$(if $1,$(firstword $1) $(call set,$(filter-out $(firstword $1),$1)))

# finds all files of a certain type in a directory recursively
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# finds all *.c files in the given directory and returns a list 
# of the names of the corresponding object files
objs=$(subst $(SRC),$1,$(patsubst %.c,%.o,$(wildcard $2/*.c)))

# takes a list of directories and returns all corresponding object files
modules=$(foreach dir,$1,$(call objs,$(OBJ),$(dir)))

# a list of all subdirectories in SRC so the compiler has access to all header files
INC := $(addprefix -I,$(call set,$(dir $(call rwildcard,$(SRC),*.h))))

ALL := $(ARBOR) $(UTIL) $(CLI) $(REVERSI) $(BAD_BATTLESHIP) $(DICE) $(RPS)

link-objects = mkdir -p $(BIN); $(CC) -o $(BIN)/$@ $+ $(LDFLAGS)

dice: $(call modules,$(ARBOR) $(DICE) $(UTIL))
	@$(link-objects)

rps: $(call modules,$(ARBOR) $(RPS) $(UTIL))
	@$(link-objects)

bad_battleship: $(call modules,$(ARBOR) $(BAD_BATTLESHIP) $(UTIL))
	@$(link-objects)

reversi: $(call modules,$(ARBOR) $(CLI) $(REVERSI) $(UTIL))
	@$(link-objects)

ut_reversi: $(call modules, $(UT_REVERSI))
	@$(link-objects)

$(OBJ)/%.o: $(SRC)/%.c
	@mkdir -p $(dir $@)
	@$(CC) $(INC) -c -o $@ $< $(CFLAGS)

all: dice bad_battleship reversi rps ut_reversi

clean:
	@rm -rf $(BLD)

.PHONY: all clean

endif # CFG in CFGS
