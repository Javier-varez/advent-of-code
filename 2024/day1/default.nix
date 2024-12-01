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

  p1 = builtins.foldl' (x: y: x + y) 0 distList;

  accumulate = l: builtins.foldl' (acc: el: acc + el) 0 l;

  eval = n1: n1 * (accumulate (map (n2: if n1 == n2 then 1 else 0) rightList));

  p2 = builtins.foldl' (acc: el: (eval el) + acc) 0 leftList;
in
{
  inherit p1 p2;
}
