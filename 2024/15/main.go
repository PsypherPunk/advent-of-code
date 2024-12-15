package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"strings"
	"time"
)

//go:embed input.txt
var input string

func draw(warehouse *map[image.Point]rune, robot image.Point) {
	maxX, maxY := 0, 0
	for position := range *warehouse {
		maxX = max(position.X, maxX)
		maxY = max(position.Y, maxY)
	}
	(*warehouse)[robot] = '@'

	fmt.Print("\033[H\033[2J")
	fmt.Println("X:", maxX, "Y:", maxY)
	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			fmt.Print(string((*warehouse)[image.Point{x, y}]))
		}
		fmt.Println()
	}

	fmt.Println()
	time.Sleep(300 * time.Millisecond)
}

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

func validWideMove(warehouse map[image.Point]rune, robot image.Point, direction image.Point) bool {
	next := robot.Add(direction)

	if warehouse[next] == ']' {
		if !validWideMove(warehouse, next, direction) {
			return false
		}

		if (direction == image.Point{0, -1}) || (direction == image.Point{0, 1}) {
			return validWideMove(warehouse, image.Point{next.X - 1, next.Y}, direction)
		}

		return true
	}

	if warehouse[next] == '[' {
		if !validWideMove(warehouse, next, direction) {
			return false
		}

		if (direction == image.Point{0, -1}) || (direction == image.Point{0, 1}) {
			return validWideMove(warehouse, image.Point{next.X + 1, next.Y}, direction)
		}

		return true
	}

	return warehouse[next] == '.'
}

func moveWider(warehouse *map[image.Point]rune, robot image.Point, direction image.Point) image.Point {
	if !validWideMove(*warehouse, robot, direction) {
		return robot
	}

	next := robot.Add(direction)
	if (*warehouse)[next] == ']' {
		moveWider(warehouse, next, direction)

		if (direction == image.Point{0, -1}) || (direction == image.Point{0, 1}) {
			moveWider(warehouse, image.Point{next.X - 1, next.Y}, direction)
		}
	}

	if (*warehouse)[next] == '[' {
		moveWider(warehouse, next, direction)

		if (direction == image.Point{0, -1}) || (direction == image.Point{0, 1}) {
			moveWider(warehouse, image.Point{next.X + 1, next.Y}, direction)
		}
	}

	(*warehouse)[next] = (*warehouse)[robot]
	(*warehouse)[robot] = '.'

	return next
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
	scanner := bufio.NewScanner(strings.NewReader(input))

	warehouse := make(map[image.Point]rune)
	robot := image.Point{}
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}

		row := []rune{}
		for x, r := range line {
			switch r {
			case '#':
				row = append(row, '#', '#')
			case 'O':
				row = append(row, '[', ']')
			case '.':
				row = append(row, '.', '.')
			case '@':
				row = append(row, '@', '.')
				robot = image.Point{x * 2, y}
			}
		}
		for x, r := range row {
			warehouse[image.Point{x, y}] = r
		}
		y++
	}

	moves := []image.Point{}
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
		robot = moveWider(&warehouse, robot, direction)
	}

	var sum int
	for position, r := range warehouse {
		if r == '[' {
			sum += 100*position.Y + position.X
		}
	}

	return sum
}

func main() {
	fmt.Println("…what is the sum of all boxes' GPS coordinates?", PartOne(input))

	fmt.Println("What is the sum of all boxes' final GPS coordinates?", PartTwo(input))
}
