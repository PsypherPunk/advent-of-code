package main

import (
	_ "embed"
	"fmt"
)

//go:embed input.txt
var input string

type coord struct {
	x int
	y int
}

var dirs = map[rune]coord{
	'^': {0, -1},
	'v': {0, 1},
	'<': {-1, 0},
	'>': {1, 0},
}

func PartOne(input string) int {
	houses := map[coord]int{{}: 1}
	current := coord{0, 0}

	for _, c := range input {
		move := dirs[c]

		current = coord{
			current.x + move.x,
			current.y + move.y,
		}

		houses[current]++
	}

	return len(houses)
}

func PartTwo(input string) int {
	houses := map[coord]int{{}: 1}
	santa := coord{0, 0}
	roboSanta := coord{0, 0}

	for i, c := range input {
		move := dirs[c]

		if i%2 == 0 {
			santa = coord{
				santa.x + move.x,
				santa.y + move.y,
			}

			houses[santa]++
		} else {
			roboSanta = coord{
				roboSanta.x + move.x,
				roboSanta.y + move.y,
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
