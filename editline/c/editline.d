/* Minix editline
 *
 * Copyright (c) 1992, 1993  Simmule Turner and Rich Salz. All rights reserved.
 *
 * This software is not subject to any license of the American Telephone
 * and Telegraph Company or of the Regents of the University of California.
 *
 * Permission is granted to anyone to use this software for any purpose on
 * any computer system, and to alter it and redistribute it freely, subject
 * to the following restrictions:
 * 1. The authors are not responsible for the consequences of use of this
 *    software, no matter how awful, even if they arise from flaws in it.
 * 2. The origin of this software must not be misrepresented, either by
 *    explicit claim or by omission.  Since few users ever read sources,
 *    credits must appear in the documentation.
 * 3. Altered versions must be plainly marked as such, and must not be
 *    misrepresented as being the original software.  Since few users
 *    ever read sources, credits must appear in the documentation.
 * 4. This notice may not be removed or altered.
 */
module lib.editline.c.editline;


private import core.stdc.stdio : FILE;


// #define CTL(x)          ((x) & 0x1F)
T CTL(T)(T x) { return x & 0x1F; }
// #define ISCTL(x)        ((x) && (x) < ' ')
dchar ISCTL(T)(T x) { return (x && x) < ' '; }
// #define UNCTL(x)        ((x) + 64)
T UNCTL(T)(T x) { return x + 64; }
// #define META(x)         ((x) | 0x80)
T META(T)(T x) { return x | 0x80; }
// #define ISMETA(x)       ((x) & 0x80)
bool ISMETA(T)(T x) { return x & 0x80; }
// #define UNMETA(x)       ((x) & 0x7F)
bool UNMETA(T)(T x) { return x & 0x7F; }


extern(C):


/* Command status codes. */
enum el_status_t {
    CSdone = 0,                 /* OK */
    CSeof,                      /* Error, or EOF */
    CSmove,
    CSdispatch,
    CSstay,
    CSsignal
}

/* Editline specific types, despite rl_ prefix.  From Heimdal project. */
alias rl_complete_func_t = char* function(char*, int*);
alias rl_list_possib_func_t = int function(char*, char***);
alias el_keymap_func_t = el_status_t function();
alias rl_hook_func_t = int function();
alias rl_getc_func_t = int function();
alias rl_voidfunc_t = void function();
alias rl_vintfunc_t = void function(int);

/* Display 8-bit chars "as-is" or as `M-x'? Toggle with M-m. (Default:0 - "as-is") */
extern int rl_meta_chars;

/* Editline specific functions. */
extern char *      el_find_word();
extern void        el_print_columns(int ac, char **av);
extern el_status_t el_ring_bell();
extern el_status_t el_del_char();

extern el_status_t el_bind_key(int key, el_keymap_func_t func);
extern el_status_t el_bind_key_in_metamap(int key, el_keymap_func_t func);

extern char       *rl_complete(char *token, int *match);
extern int         rl_list_possib(char *token, char ***av);

/* For compatibility with FSF readline. */
// TODO FIX THIS
extern __gshared int         rl_point;
extern __gshared int         rl_mark;
extern __gshared int         rl_end;
extern __gshared int         rl_inhibit_complete;
extern __gshared char       *rl_line_buffer;
extern __gshared const(char)*rl_readline_name;
extern __gshared FILE       *rl_instream;  /* The stdio stream from which input is read. Defaults to stdin if NULL - Not supported yet! */
extern __gshared FILE       *rl_outstream; /* The stdio stream to which output is flushed. Defaults to stdout if NULL - Not supported yet! */
extern __gshared int         el_no_echo;   /* E.g under emacs, don't echo except prompt */
extern __gshared int         el_no_hist;   /* Disable auto-save of and access to history -- e.g. for password prompts or wizards */
extern __gshared int         el_hist_size; /* size of history scrollback buffer, default: 15 */

extern void rl_initialize();
extern void rl_reset_terminal(const(char)* terminal_name);

void rl_save_prompt();
void rl_restore_prompt();
void rl_set_prompt(const(char)* prompt);

void rl_clear_message();
void rl_forced_update_display();

extern char *readline(const(char)* prompt);
extern void add_history(const(char)* line);

extern int read_history(const(char)* filename);
extern int write_history(const(char)* filename);

rl_complete_func_t    *rl_set_complete_func(rl_complete_func_t *func);
rl_list_possib_func_t *rl_set_list_possib_func(rl_list_possib_func_t *func);

void rl_prep_terminal(int meta_flag);
void rl_deprep_terminal();

int rl_getc();
