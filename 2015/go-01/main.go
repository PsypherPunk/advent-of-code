package main

import (
	_ "embed"
	"fmt"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	floor := 0

	for _, c := range input {
		switch c {
		case '(':
			floor++
		case ')':
			floor--
		}
	}

	return floor
}

func PartTwo(input string) int {
	var position int
	floor := 0

	for i, c := range input {
		switch c {
		case '(':
			floor++
		case ')':
			floor--
		}

		if floor == -1 {
			position = i + 1
			break
		}
	}

	return position
}

func main() {
	fmt.Println("To what floor do the instructions take Santa?", PartOne(input))

	fmt.Println("What is the position of the character that causes Santa to first enter the basement?", PartTwo(input))
}
