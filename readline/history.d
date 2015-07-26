module lib.readline.history;


private import std.string : toStringz;
public import lib.readline.c.history;


alias add_history = lib.readline.c.history.add_history;

alias read_history = lib.readline.c.history.read_history;
alias write_history = lib.readline.c.history.write_history;


void add_history(const(char)[] input)
{
	auto s = toStringz(input);
	add_history(s);
}

int read_history(const(char)[] filename)
{
	auto s = toStringz(filename);
	return read_history(s);
}

int write_history(const(char)[] filename)
{
	auto s = toStringz(filename);
	return read_history(s);
}