module lib.readline.c.keymap;


public import lib.readline.c.rltypedefs;


extern (C):


struct KEYMAP_ENTRY {
  char type;
  rl_command_func_t func;
}

alias Keymap = KEYMAP_ENTRY*;

enum KEYMAP_SIZE = 257;
