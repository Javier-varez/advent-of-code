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

func hasSolution(line Line, idx int, acc int64) bool {
	if idx == len(line.values) {
		return acc == line.result
	}

	return hasSolution(line, idx+1, acc+line.values[idx]) || hasSolution(line, idx+1, acc*line.values[idx])
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
		if hasSolution(eq, 1, eq.values[0]) {
			solution += eq.result
		}
	}

	fmt.Println("solution ", solution)

}
