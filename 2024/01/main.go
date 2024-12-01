package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"sort"
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

func PartOne(input string) int {
	var left, right []int

	scanner := bufio.NewScanner(strings.NewReader(strings.TrimSpace(input)))
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)
		a, errA := strconv.Atoi(parts[0])
		b, errB := strconv.Atoi(parts[1])
		if errA != nil || errB != nil {
			fmt.Println("Invalid line:", line, errA, errB)
			continue
		}

		left = append(left, a)
		right = append(right, b)
	}

	sort.Ints(left)
	sort.Ints(right)

	distance := 0
	for i := 0; i < len(left); i++ {
		distance += abs(left[i], right[i])
	}

	return distance
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What is the total distance between your lists?", PartOne(input))
}
