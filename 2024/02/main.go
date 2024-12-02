package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func abs(x, y int) int {
	if x < y {
		return y - x
	}
	return x - y
}

func isIncreasing(reports []int) bool {
	for i := 0; i < len(reports)-1; i++ {
		if reports[i] >= reports[i+1] {
			return false
		}
	}
	return true
}

func isDecreasing(reports []int) bool {
	for i := 0; i < len(reports)-1; i++ {
		if reports[i] <= reports[i+1] {
			return false
		}
	}
	return true
}

func isSafeDiff(reports []int) bool {
	for i := 0; i < len(reports)-1; i++ {
		diff := abs(reports[i], reports[i+1])
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func IsSafe(line string) bool {
	fields := strings.Fields(line)
	reports := make([]int, len(fields))

	for i, field := range fields {
		report, err := strconv.Atoi(field)
		if err != nil {
			fmt.Printf("Error converting %s to int: %v\n", field, err)
		}
		reports[i] = report
	}

	return (isIncreasing(reports) || isDecreasing(reports)) && isSafeDiff(reports)
}

func IsSafeButTolerant(report string) bool {
	return false
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	safe := 0
	for scanner.Scan() {
		if IsSafe(scanner.Text()) {
			safe++
		}
	}

	return safe
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many reports are safe?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
