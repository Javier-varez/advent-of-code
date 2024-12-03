let input = open ./input.txt;

def parseSingleMultiplication [] {
    $in | parse --regex 'mul\((?P<first>[0-9]*),(?P<second>[0-9]*)\)' | update first { $in | into int } | update second { $in | into int }
}

let multiplications = $input | parse --regex '(?P<muls>mul\([0-9]*,[0-9]*\))' | get muls | each {|i| $i | parseSingleMultiplication };

let multiplyAccumulate = $multiplications | each {|i| $i.first.0 * $i.second.0 } | reduce {|it, acc| $it + $acc };

$multiplyAccumulate
