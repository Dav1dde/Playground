module lib.readline.c.history;


// Also very incomplete

extern (C):


void add_history(const(char)*);

void clear_history();

int read_history(const(char)*);
int write_history(const(char)*);