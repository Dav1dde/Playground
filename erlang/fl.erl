-module(fl).

%% API
-export([flatten/1]).


flatten(X) ->
  flatten(X, []).

flatten([H | T], L) ->
  flatten(H, [T | L]);

flatten([], [H | T]) ->
  flatten(H, T);

flatten([], []) ->
  [];

flatten(E, L) ->
  if
    is_tuple(E) -> flatten(tuple_to_list(E), L);
    true -> [E | flatten(L, [])]
  end.