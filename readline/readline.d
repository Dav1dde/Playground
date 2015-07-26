module lib.readline.readline;


private {
	import core.stdc.stdlib : free;
	import std.string : toStringz;
	import std.conv : to;
}
public import lib.readline.c.readline;
public import lib.readline.c.chardefs;
public import lib.readline.c.keymaps;


alias readline = lib.readline.c.readline.readline;
alias rl_replace_line = lib.readline.c.readline.rl_replace_line;
alias rl_insert_text = lib.readline.c.readline.rl_insert_text;
alias rl_bind_key = lib.readline.c.readline.rl_bind_key;


string readline(const(char)[] prompt)
{
	auto s = readline(toStringz(prompt));
	scope(exit) free(s);
	if (s is null) {
		return null;
	}
	string r = to!string(s);
	return r is null ? "" : r;
}


void rl_replace_line(const(char)[] text, int clear_undo)
{
	rl_replace_line(toStringz(text), clear_undo);
}

int rl_insert_text(const(char)[] text)
{
	return rl_insert_text(toStringz(text));
}


private __gshared void delegate(int, int)[KEYMAP_SIZE] _rl_bind_key_dgs;
extern(C) int _rl_bind_key_cb(int x, int key)
{
	// you can only bind to keys which are in range
	auto dg = _rl_bind_key_dgs[key];
	if (dg !is null) {
		dg(x, key);
	}
	return 0;
}


int rl_bind_key(int key, void delegate(int, int) dg)
{
	auto r = lib.readline.c.readline.rl_bind_key(key, &_rl_bind_key_cb);
	if (r == 0) {
		// rl_bind_key alread checks if the key is in range
		_rl_bind_key_dgs[key] = dg;
	}
	return r;
}

int rl_unbind_key(int key)
{
	auto r = lib.readline.c.readline.rl_unbind_key(key);
	if (r == 0) {
		_rl_bind_key_dgs[key] = null;
	}
	return r;
}