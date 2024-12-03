let input = open ./input.txt;

def parseSingleMultiplication [] {
    $in | parse --regex 'mul\((?P<first>[0-9]*),(?P<second>[0-9]*)\)' | $in.0 | update first { into int } | update second { into int }
}

let multiplications = $input | parse --regex '(?P<muls>mul\([0-9]*,[0-9]*\))' | get muls | each { parseSingleMultiplication };

let multiplyAccumulate = $multiplications | each {|i| $i.first * $i.second } | reduce {|it, acc| $it + $acc };

$multiplyAccumulate
