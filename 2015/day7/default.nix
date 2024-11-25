let
  pkgs = import <nixpkgs> { };
  inherit (pkgs) lib;

  input = lib.readFile ./input.txt;
  inputLines = builtins.filter (s: (builtins.stringLength s) > 0) (lib.splitString "\n" input);

  parseOperation =
    op:
    let
      parts = lib.splitString " " op;
      opcode =
        if (lib.length parts) == 1 then
          "IDENT"
        else if (lib.length parts) == 2 then
          (lib.head parts)
        else
          lib.elemAt parts 1;
      args =
        if (lib.length parts) == 1 then
          [ (lib.head parts) ]
        else if (lib.length parts) == 2 then
          [ (lib.elemAt parts 1) ]
        else
          [
            (lib.head parts)
            (lib.elemAt parts 2)
          ];
    in
    {
      inherit opcode args;
    };

  parseInstr =
    inst:
    let
      parts = map lib.trim (lib.splitString "->" inst);
      operands = builtins.head parts;
      dest = builtins.elemAt parts 1;
    in
    {
      inherit dest;
    }
    // (parseOperation operands);

  instList = lib.map parseInstr inputLines;

  rules = builtins.listToAttrs (
    map (inst: {
      name = inst.dest;
      value = {
        inherit (inst) opcode args;
      };
    }) instList
  );

  resolve =
    solved: currentRule:
    let
      stringIsNumber = str: (builtins.tryEval (lib.toInt str)).success;
      findVar = varName: if stringIsNumber varName then lib.toInt varName else solved."${varName}";
    in
    if currentRule.opcode == "IDENT" then
      findVar (lib.head currentRule.args)
    else if currentRule.opcode == "NOT" then
      let
        varName = lib.head currentRule.args;
        var = findVar varName;
      in
      lib.bitNot var
    else
      let
        lvarName = lib.head currentRule.args;
        lvar = findVar lvarName;
        rvarName = lib.elemAt currentRule.args 1;
        rvar = findVar rvarName;

        result =
          if currentRule.opcode == "AND" then
            builtins.bitAnd lvar rvar
          else if currentRule.opcode == "OR" then
            builtins.bitOr lvar rvar
          else if currentRule.opcode == "LSHIFT" then
            lshift lvar rvar
          else if currentRule.opcode == "RSHIFT" then
            rshift lvar rvar
          else
            throw "Unknown instruction ${currentRule.opcode}";
      in
      result;

  lshift = n: shift: if shift == 0 then builtins.bitAnd n 65535 else lshift (n * 2) (shift - 1);

  rshift = n: shift: if shift == 0 then builtins.bitAnd n 65535 else rshift (n / 2) (shift - 1);

  solveStep =
    rules: solved:
    let
      canBeSolved =
        varName:
        let
          stringIsNumber = str: (builtins.tryEval (lib.toInt str)).success;
          isVarSolved = var: (builtins.hasAttr var solved);
          isVarSolvedOrConstant = var: ((isVarSolved var) || (stringIsNumber var));
          inputsAreSolved = var: builtins.all isVarSolvedOrConstant (rules."${var}".args);
        in
        !(isVarSolved varName) && (inputsAreSolved varName);
      toSolve = lib.findFirst canBeSolved "" (builtins.attrNames rules);
    in
    solved
    // {
      "${toSolve}" = resolve solved rules."${toSolve}";
    };

  resolveOrdered =
    rules: solved: varName:
    if builtins.hasAttr varName solved then
      solved."${varName}"
    else
      resolveOrdered rules (solveStep rules solved) varName;

  eval = var: resolveOrdered rules { } var;

  overridenRules = rules // {
    "b" = {
      opcode = "IDENT";
      args = [ "${builtins.toString (eval "a")}" ];
    };
  };
  evalPart2 = var: resolveOrdered overridenRules { } var;

in
{
  p1 = eval "a";
  p2 = evalPart2 "a";
}
