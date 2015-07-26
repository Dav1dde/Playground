module lib.readline.c.readline;


public import lib.readline.c.rltypedefs;
public import lib.readline.c.keymaps;

// This is very incomplete at the moment.

@nogc nothrow @system:
extern (C):


char* readline(const(char)*);

/* Modifying text. */
void rl_replace_line(const(char)*, int);
int rl_insert_text(const(char)*);
int rl_delete_text(int, int);
int rl_kill_text(int, int);
char *rl_copy_text(int, int);

/* Utility functions to bind keys to readline commands. */
int rl_bind_key(int, rl_command_func_t);
int rl_bind_keyseq(const(char)*, rl_command_func_t);
int rl_unbind_key(int);

Keymap rl_get_keymap();

void rl_tty_set_default_bindings(Keymap);
void rl_tty_unset_default_bindings(Keymap);

/* Variables */
extern __gshared rl_hook_func_t rl_startup_hook;

extern __gshared rl_hook_func_t rl_event_hook;

extern __gshared int rl_done;
