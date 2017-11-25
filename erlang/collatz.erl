-module(collatz).

%% API
-export([collatz/1]).


% even
collatz(N) when N > 1, N rem 2 =:= 0 ->
  [N | collatz(N div 2)];

% odd
collatz(N) when N > 1, N rem 2 =/= 0 ->
  [N | collatz(N * 3 + 1)];

collatz(N) when is_integer(N) ->
  [N].
