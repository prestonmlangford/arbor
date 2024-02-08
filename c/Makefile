SHELL 			:= bash
CC 				:= clang

OUT 			:= arbor-cli

ROOT 			:= $(PWD)
SRC 			:= $(ROOT)/src
BLD 			:= $(ROOT)/build

ARBOR 			:= $(SRC)/arbor
UTIL 			:= $(SRC)/util
CLI 			:= $(SRC)/cli

GAMES 			:= $(SRC)/games
REVERSI 		:= $(GAMES)/reversi
BAD_BATTLESHIP 	:= $(GAMES)/bad_battleship
DICE 			:= $(GAMES)/dice
RPS 			:= $(GAMES)/rps

CFLAGS			:=

ifdef cfg
include cfg/$(cfg).mk
else
include cfg/release.mk
endif

# removes duplicates in a list
set=$(if $1,$(firstword $1) $(call set,$(filter-out $(firstword $1),$1)))

# finds all files of a certain type in a directory recursively
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# finds all *.c files in the given directory and returns a list 
# of the names of the corresponding object files
objs=$(subst $(SRC),$1,$(patsubst %.c,%.o,$(wildcard $2/*.c)))

# takes a list of directories and returns all corresponding object files
modules=$(foreach dir,$1,$(call objs,$(BLD)/$(CFG),$(dir)))

# a list of all subdirectories in SRC so the compiler has access to all header files
INC := $(addprefix -I,$(call set,$(dir $(call rwildcard,$(SRC),*.h))))

ALL := $(ARBOR) $(UTIL) $(CLI) $(REVERSI) $(BAD_BATTLESHIP) $(DICE) $(RPS)

$(OUT): $(call modules,$(ALL))
	@$(CC) -o $(BLD)/$(CFG)/$(OUT) $+

$(BLD)/$(CFG)/%.o: $(SRC)/%.c
	@mkdir -p $(dir $@)
	@$(CC) $(INC) -c -o $@ $< $(CFLAGS)

clean:
	@rm -rf $(BLD)

.PHONY: all clean
