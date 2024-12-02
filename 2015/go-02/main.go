package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"slices"
	"strings"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	var feet int

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		var l, w, h int

		_, err := fmt.Sscanf(line, "%dx%dx%d", &l, &w, &h)
		if err != nil {
			fmt.Println(line)
			panic(err)
		}

		sides := []int{l * w, w * h, h * l}
		smallest := slices.Min(sides)

		for i := range sides {
			feet += sides[i] * 2
		}
		feet += smallest
	}

	return feet
}

func PartTwo(input string) int {
	var feet int

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		var l, w, h int

		_, err := fmt.Sscanf(line, "%dx%dx%d", &l, &w, &h)
		if err != nil {
			fmt.Println(line)
			panic(err)
		}

		edges := []int{
			2 * (l + w),
			2 * (w + h),
			2 * (l + h),
		}
		feet += slices.Min(edges)

		feet += l * w * h
	}

	return feet
}

func main() {
	fmt.Println("All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?", PartOne(input))

	fmt.Println("How many total feet of ribbon should they order?", PartTwo(input))
}
