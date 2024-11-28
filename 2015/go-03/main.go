package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

var dirs = map[rune][2]int{
	'^': [2]int{0, -1},
	'v': [2]int{0, 1},
	'<': [2]int{-1, 0},
	'>': [2]int{1, 0},
}

func PartOne(input string) int {
	houses := map[[2]int]int{[2]int{}: 1}
	current := [2]int{0, 0}

	for _, c := range strings.TrimSuffix(input, "\n") {
		move := dirs[c]

		current = [2]int{
			current[0] + move[0],
			current[1] + move[1],
		}

		houses[current]++
	}

	return len(houses)
}

func PartTwo(input string) int {
	houses := map[[2]int]int{[2]int{}: 1}
	santa := [2]int{0, 0}
	roboSanta := [2]int{0, 0}

	for i, c := range strings.TrimSuffix(input, "\n") {
		move := dirs[c]

		if i%2 == 0 {
			santa = [2]int{
				santa[0] + move[0],
				santa[1] + move[1],
			}

			houses[santa]++
		} else {
			roboSanta = [2]int{
				roboSanta[0] + move[0],
				roboSanta[1] + move[1],
			}

			houses[roboSanta]++
		}
	}

	return len(houses)
}

func main() {
	fmt.Println("How many houses receive at least one present?", PartOne(input))

	fmt.Println("This year, how many houses receive at least one present?", PartTwo(input))
}
