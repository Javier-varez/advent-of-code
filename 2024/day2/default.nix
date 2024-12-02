let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib;

  input = lib.readFile ./input.txt;
  data = map (line: map (e: lib.toInt e) (lib.splitString " " line)) (
    lib.filter (l: l != "") (lib.splitString "\n" input)
  );

  isMonotonic =
    increasing: l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result = (if increasing then cur > prev else cur < prev) && result;
        }
      )
      {
        prev = if increasing then -1 else 100;
        result = true;
      }
      l
    ).result;

  isIncreasing = isMonotonic true;
  isDecreasing = isMonotonic false;

  abs = v: if v < 0 then -v else v;

  distance =
    max: l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result =
            let
              distance = abs (cur - prev);
              condition = if max then distance > result else distance < result;
            in
            if condition then distance else result;
        }
      )
      {
        prev = builtins.head l;
        result = if max then 0 else 100;
      }
      (lib.lists.sublist 1 ((lib.count (e: true) l) - 1) l)
    ).result;

  maxDistance = distance true;
  minDistance = distance false;

  isLevelSafe =
    l: ((isIncreasing l) || (isDecreasing l)) && ((minDistance l) >= 1) && ((maxDistance l) <= 3);

  safeLevels = map (l: isLevelSafe l) data;
  numSafeLevels = lib.count (safe: safe) safeLevels;

in
numSafeLevels
