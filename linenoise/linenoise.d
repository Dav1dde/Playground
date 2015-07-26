/**
 *
 * The module is supposed to be imported like this:
 *
 * import linenoise = lib.linenoise.linenose;
 *
 */
module lib.linenoise.linenoise;


private {
	import core.stdc.stdlib : free;
	import std.conv : to;
	import std.string : toStringz;
}
public import lib.linenoise.c.linenoise;


bool line(const(char)[] prompt, out string line)
{
	auto r = linenoise(toStringz(prompt));
	// freeing null is fine
	scope (exit) free(r);

	line = to!string(r);

	return r is null;
}


bool addHistory(const(char)[] line)
{
	return linenoiseHistoryAdd(toStringz(line)) == 1;
}


bool setHistoryCapacity(int size)
{
	return linenoiseHistorySetMaxLen(size) == 1;
}


bool saveHistory(const(char)[] filename)
{
	return linenoiseHistorySave(toStringz(filename)) == 0;
}


bool loadHistory(const(char)[] filename)
{
	return linenoiseHistoryLoad(toStringz(filename)) == 0;
}


alias clearScreen = linenoiseClearScreen;


void setMultiLine(bool ml)
{
	linenoiseSetMultiLine(cast(int)ml);
}


alias printKeyCodes = linenoisePrintKeyCodes;


private static __gshared void* _data;
alias CompletionData = linenoiseCompletions*;
alias CompletionCallback = void function(string, CompletionData lc, void* data);
private static __gshared  CompletionCallback _cb;


extern(C) void _linenoiseCallback(const(char)* buf, linenoiseCompletions* lc)
{
	_cb(to!string(buf), lc, _data);
}


void setCompletionCallback(CompletionCallback cb, void* data)
{
	_data = data;
	_cb = cb;
	linenoiseSetCompletionCallback(&_linenoiseCallback);
}


void addCompletion(CompletionData lc, string completion)
{
	linenoiseAddCompletion(lc, toStringz(completion));
}

