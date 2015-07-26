module lib.editline.editline;


private {
	import core.stdc.stdlib : free;
	import std.string : toStringz;
	import std.conv : to;
}
public import lib.editline.c.editline;


alias rl_reset_terminal = lib.editline.c.editline.rl_reset_terminal;
alias rl_set_prompt = lib.editline.c.editline.rl_set_prompt;
alias readline = lib.editline.c.editline.readline;
alias add_history = lib.editline.c.editline.add_history;
alias read_history = lib.editline.c.editline.read_history;
alias write_history = lib.editline.c.editline.write_history;


void rl_reset_terminal(const(char)[] terminal_name)
{
	lib.editline.c.editline.rl_reset_terminal(toStringz(terminal_name));
}


void rl_set_prompt(const(char)[] prompt)
{
	lib.editline.c.editline.rl_set_prompt(toStringz(prompt));
}


string readline(const(char)[] prompt)
{
	auto s = lib.editline.c.editline.readline(toStringz(prompt));
	scope(exit) free(s);
	if (s is null) {
		return null;
	}
	string r = to!string(s);
	return r is null ? "" : r;
}


void add_history(const(char)[] line)
{
	lib.editline.c.editline.add_history(toStringz(line));
}


int read_history(const(char)[] filename)
{
	return lib.editline.c.editline.read_history(toStringz(filename));
}


int write_history(const(char)[] filename)
{
	return lib.editline.c.editline.write_history(toStringz(filename));
}