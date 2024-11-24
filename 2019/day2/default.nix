let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib;
  input = lib.readFile ./input.txt;
  input_list = map lib.toInt (lib.splitString "," (lib.removeSuffix "\n" input));

  replace =
    list: dest: newValue:
    let
      len = lib.lists.count (e: true) list;
      left = lib.lists.sublist 0 dest list;
      right = lib.lists.sublist (dest + 1) (len - dest) list;
    in
    left ++ [ newValue ] ++ right;

  process =
    list: idx:
    let
      opcode = (builtins.elemAt list idx);
    in
    if opcode == 99 then
      list
    else
      let
        firstArg = builtins.elemAt list (idx + 1);
        secondArg = builtins.elemAt list (idx + 2);
        dest = idx + 3;
        op = if opcode == 1 then (a: b: a + b) else (a: b: a * b);
        newValue = op (builtins.elemAt list firstArg) (builtins.elemAt list secondArg);
        newList = replace list (builtins.elemAt list dest) newValue;
      in
      process newList (idx + 4);

  runForNounAndVerb =
    noun: verb:
    let
      list = replace (replace input_list 1 noun) 2 verb;
    in
    builtins.elemAt (process list 0) 0;

  verbAndNoun =
    let
      nouns = lib.range 0 99;
      verbs = lib.range 0 99;
      allNounVerbCombinations = lib.flatten (
        builtins.map (noun: builtins.map (verb: { inherit noun verb; }) verbs) nouns
      );
    in
    builtins.foldl' (
      acc: verbAndNoun:
      let
        cur_result = runForNounAndVerb verbAndNoun.noun verbAndNoun.verb;
      in
      if ((builtins.hasAttr "verb" acc) && (builtins.hasAttr "noun" acc)) || cur_result != 19690720 then
        acc
      else
        verbAndNoun
    ) { } allNounVerbCombinations;

  p1 = builtins.elemAt (process input_list 0) 0;
  p2 = verbAndNoun.noun * 100 + verbAndNoun.verb;
in
{
  inherit p1 p2;
}
