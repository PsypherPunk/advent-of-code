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

type Guard struct {
	steps     map[image.Point]bool
	position  image.Point
	direction image.Point
}

func draw(obstructions map[image.Point]bool, guard Guard, maxX int, maxY int) {
	fmt.Println()
	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			var r rune

			_, ok := obstructions[image.Point{x, y}]
			if ok {
				r = '#'
			} else if guard.position.Eq(image.Point{x, y}) {
				r = 'O'
			} else {
				r = '.'
			}

			fmt.Print(string(r))
		}
		fmt.Println()
	}
}

func step(obstructions map[image.Point]bool, guard *Guard, x int, y int) bool {
	next := guard.position.Add(guard.direction)

	if next.X < 0 || next.Y < 0 || next.X > x || next.Y > y {
		return true
	}

	_, ok := obstructions[next]
	if ok {
		guard.direction = image.Point{-guard.direction.Y, guard.direction.X}
	} else {
		guard.position = next
		guard.steps[next] = true
	}

	return false
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var guard Guard
	obstructions := make(map[image.Point]bool)
	var maxX, maxY int

	y := 0
	for scanner.Scan() {
		maxY = y
		for x, c := range scanner.Text() {
			switch c {
			case '^':
				guard = Guard{map[image.Point]bool{{x, y}: true}, image.Point{x, y}, image.Point{0, -1}}
			case '#':
				obstructions[image.Point{x, y}] = true
			}
			maxX = x
		}
		y++
	}

	// fmt.Print("\033[H\033[2J")
	for !step(obstructions, &guard, maxX, maxY) {
		// draw(obstructions, guard, maxX, maxY)
		// time.Sleep(50 * time.Millisecond)
		// fmt.Print("\033[H\033[2J")
	}

	return len(guard.steps)
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many distinct positions will the guard visit before leaving the mapped area?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
