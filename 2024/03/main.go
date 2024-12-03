package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	sum := 0
	re := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	scanner := bufio.NewScanner(strings.NewReader(input))

	for scanner.Scan() {
		line := scanner.Text()

		matches := re.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			a, errA := strconv.Atoi(match[1])
			b, errB := strconv.Atoi(match[2])

			if errA != nil || errB != nil {
				fmt.Println("Invalid line:", line)
				continue
			}

			sum += a * b
		}
	}

	return sum
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What do you get if you add up all of the results of the multiplications?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
