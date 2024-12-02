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

  generateOptions =
    l:
    let
      numLevels = lib.count (e: true) l;
      generateLevel = i: ((lib.sublist 0 i l) ++ (lib.sublist (i + 1) numLevels l));
      generateLevelRec =
        i: levels:
        let
          newLevels = levels ++ [ (generateLevel i) ];
        in
        if i > 0 then (generateLevelRec (i - 1) newLevels) else newLevels;
    in
    generateLevelRec (numLevels - 1) [ ];

  safeLevelsWithRemovals = map (l: lib.any (safe: safe) (map (l: isLevelSafe l) (generateOptions l))) data;
  numSafeLevelsWithRemovals = lib.count (safe: safe) safeLevelsWithRemovals;

in
{
  p1 = numSafeLevels;
  p2 = numSafeLevelsWithRemovals;
}
