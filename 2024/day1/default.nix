let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib;
  inputTxt = lib.readFile ./input.txt;
  inputLines = lib.filter (l: (lib.stringLength l) > 0) (lib.splitString "\n" inputTxt);

  leftList =
    let
      filtered = builtins.map (line: lib.head (lib.splitString " " line)) inputLines;
    in
    builtins.map lib.toInt filtered;

  rightList =
    let
      filtered = builtins.map (line: lib.elemAt (lib.splitString " " line) 3) inputLines;
    in
    builtins.map lib.toInt filtered;

  leftSorted = builtins.sort (a: b: a < b) leftList;
  rightSorted = builtins.sort (a: b: a < b) rightList;

  abs = n: if n < 0 then -n else n;

  distList = map ({ fst, snd }: abs (fst - snd)) (lib.zipLists leftSorted rightSorted);

  reduced = builtins.foldl' (x: y: x + y) 0 distList;
in
reduced
