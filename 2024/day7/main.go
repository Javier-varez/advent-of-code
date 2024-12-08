package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Line struct {
	result int64
	values []int64
}

func digits(b int64) int {
	order := 0
	for b > 0 {
		order += 1
		b /= 10
	}
	return order
}

func pow10(order int) int64 {
	val := int64(1)
	for i := 0; i < order; i++ {
		val *= 10
	}
	return val
}

func concat(a, b int64) int64 {
	order := digits(b)
	return a*pow10(order) + b
}

func hasSolution(line Line, idx int, acc int64, useConcat bool) bool {
	if idx == len(line.values) {
		return acc == line.result
	}

	return hasSolution(line, idx+1, acc+line.values[idx], useConcat) ||
		hasSolution(line, idx+1, acc*line.values[idx], useConcat) ||
		(useConcat && hasSolution(line, idx+1, concat(acc, line.values[idx]), useConcat))
}

func main() {
	buffer, err := os.ReadFile("./realinput.txt")
	if err != nil {
		log.Fatal(err)
	}

	s := bufio.NewScanner(strings.NewReader(string(buffer)))
	s.Buffer(buffer, len(buffer))

	var lines []Line
	for s.Scan() {
		str := s.Text()
		if str == "" {
			continue
		}

		parts := strings.Split(str, ":")
		result, err := strconv.ParseInt(parts[0], 0, 64)
		if err != nil {
			log.Fatal(err)
		}

		var values []int64
		for _, val := range strings.Split(strings.TrimSpace(parts[1]), " ") {
			parsed, err := strconv.ParseInt(strings.TrimSpace(val), 0, 64)
			if err != nil {
				log.Fatal(err)
			}
			values = append(values, parsed)
		}
		lines = append(lines, Line{result: result, values: values})
	}

	solution := int64(0)
	for _, eq := range lines {
		if hasSolution(eq, 1, eq.values[0], false) {
			solution += eq.result
		}
	}

	solution2 := int64(0)
	for _, eq := range lines {
		if hasSolution(eq, 1, eq.values[0], true) {
			solution2 += eq.result
		}
	}
	fmt.Println("solution 1", solution)
	fmt.Println("solution 2", solution2)

}
