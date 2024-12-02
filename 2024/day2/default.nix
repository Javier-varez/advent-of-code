let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib;

  input = lib.readFile ./input.txt;
  data = map (line: map (e: lib.toInt e) (lib.splitString " " line)) (
    lib.filter (l: l != "") (lib.splitString "\n" input)
  );

  isIncreasing =
    l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result = (cur > prev) && result;
        }
      )
      {
        prev = -1;
        result = true;
      }
      l
    ).result;

  isDecreasing =
    l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result = (cur < prev) && result;
        }
      )
      {
        prev = 100;
        result = true;
      }
      l
    ).result;

  abs = v: if v < 0 then -v else v;

  minDistance =
    l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result =
            let
              distance = abs (cur - prev);
            in
            if distance < result then distance else result;
        }
      )
      {
        prev = builtins.head l;
        result = 100;
      }
      (lib.lists.sublist 1 ((lib.count (e: true) l) - 1) l)
    ).result;

  maxDistance =
    l:
    (lib.foldl'
      (
        { prev, result }:
        cur: {
          prev = cur;
          result =
            let
              distance = abs (cur - prev);
            in
            if distance > result then distance else result;
        }
      )
      {
        prev = builtins.head l;
        result = 0;
      }
      (lib.lists.sublist 1 ((lib.count (e: true) l) - 1) l)
    ).result;

  isLevelSafe =
    l: ((isIncreasing l) || (isDecreasing l)) && ((minDistance l) >= 1) && ((maxDistance l) <= 3);

  safeLevels = map (l: isLevelSafe l) data;
  numSafeLevels = lib.count (safe: safe) safeLevels;

in
numSafeLevels
