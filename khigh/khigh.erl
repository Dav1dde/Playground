-module(khigh).

-export([khigh/1]).


khigh(L) ->
  khigh(L, 0).


khigh([H | Rest], Level) ->
  if
    H == $( -> khigh(Rest, Level+1);
    H == $) -> khigh(Rest, max(Level-1, 0));
    Level > 0 -> khigh_format(H, Rest) ++ khigh(Rest, Level);
    true -> [H | khigh(Rest, Level)]
  end;

khigh([], _Level) ->
  [].


khigh_format(C, [H | _]) ->
  if
    H == $) -> [C];
    H == $  -> [C];
    C == $  -> [C];
    true -> [C, $_]
  end;

khigh_format(C, []) ->
  C.
