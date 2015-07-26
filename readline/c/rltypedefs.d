module lib.readline.c.rltypedefs;


public import core.stdc.stdio : FILE;


extern (C):

/* Bindable functions */
alias rl_command_func_t = int function(int, int);

/* Typedefs for the completion system */
alias rl_compentry_func_t  = char * function(const char *, int);
alias rl_completion_func_t = char ** function(const char *, int, int);

alias rl_quote_func_t = char * function(char *, int, char *);
alias rl_dequote_func_t = char * function(char *, int);

alias rl_compignore_func_t = int function(char **);

alias rl_compdisp_func_t = void function(char **, int, int);

/* Type for input and pre-read hook functions like rl_event_hook */
alias rl_hook_func_t = int function();

/* Input function type */
alias rl_getc_func_t = int function(FILE *);

/* Generic function that takes a character buffer (which could be the readline
   line buffer) and an index into it (which could be rl_point) and returns
   an int. */
alias rl_linebuf_func_t = int function(char *, int);

/* `Generic' function pointer typedefs */
alias rl_intfunc_t = int function(int);
alias rl_ivoidfunc_t = rl_hook_func_t;
alias rl_icpfunc_t = int function(char *);
alias rl_icppfunc_t = int function(char **);

alias rl_voidfunc_t = void function();
alias rl_vintfunc_t = void function(int);
alias rl_vcpfunc_t = void function(char *);
alias rl_vcppfunc_t = void function(char **);

alias rl_cpvfunc_t = char * function();
alias rl_cpifunc_t = char * function(int);
alias rl_cpcpfunc_t = char * function(char  *);
alias rl_cpcppfunc_t = char * function(char  **);
