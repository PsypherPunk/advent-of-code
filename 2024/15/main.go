package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"strings"
)

//go:embed input.txt
var input string

func move(warehouse *map[image.Point]rune, robot image.Point, direction image.Point) image.Point {
	next := robot.Add(direction)

	if (*warehouse)[next] == 'O' {
		move(warehouse, next, direction)
	}

	if (*warehouse)[next] == '.' {
		(*warehouse)[next] = (*warehouse)[robot]
		(*warehouse)[robot] = '.'
		return next
	}

	return robot
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	warehouse := make(map[image.Point]rune)
	moves := []image.Point{}
	var robot image.Point
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}

		for x, r := range line {
			warehouse[image.Point{x, y}] = r
			if r == '@' {
				robot = image.Point{x, y}
			}
		}
		y++
	}

	for scanner.Scan() {
		line := scanner.Text()
		for _, r := range line {
			switch r {
			case '^':
				moves = append(moves, image.Point{0, -1})
			case '>':
				moves = append(moves, image.Point{1, 0})
			case 'v':
				moves = append(moves, image.Point{0, 1})
			case '<':
				moves = append(moves, image.Point{-1, 0})
			default:
				fmt.Println("invalid line: ", line)
				continue
			}
		}
	}

	for _, direction := range moves {
		robot = move(&warehouse, robot, direction)
	}

	gpsSum := 0
	for position, r := range warehouse {
		if r == 'O' {
			gpsSum += position.X + (100 * position.Y)
		}
	}

	return gpsSum
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("…what is the sum of all boxes' GPS coordinates?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
