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
	steps     map[image.Point]image.Point
	position  image.Point
	direction image.Point
}

type Exit int

const (
	InMap    Exit = 0
	OutOfMap Exit = 1
	Loop     Exit = 2
)

func draw(obstructions map[image.Point]bool, guard Guard, maxX int, maxY int) {
	fmt.Println()
	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			var r rune

			_, ok := obstructions[image.Point{x, y}]
			if ok {
				r = '#'
			} else if guard.position.Eq(image.Point{x, y}) {
				switch guard.direction {
				case image.Point{0, -1}:
					r = '^'
				case image.Point{1, 0}:
					r = '>'
				case image.Point{0, 1}:
					r = 'V'
				case image.Point{-1, 0}:
					r = '<'
				}
			} else {
				r = '.'
			}

			fmt.Print(string(r))
		}
		fmt.Println()
	}
}

func stepInArea(obstructions map[image.Point]bool, guard *Guard, x int, y int) Exit {
	next := guard.position.Add(guard.direction)

	if next.X < 0 || next.Y < 0 || next.X > x || next.Y > y {
		return OutOfMap
	}

	_, ok := obstructions[next]
	if ok {
		// "If there is something directly in front of you, turn right 90 degrees."
		guard.direction = image.Point{-guard.direction.Y, guard.direction.X}
	} else {
		// "Otherwise, take a step forward."
		guard.position = next
		direction, ok := guard.steps[next]
		// "…such a way that the guard will get stuck in a loop…"
		if ok && direction.Eq(guard.direction) {
			return Loop
		}
		guard.steps[next] = guard.direction
	}

	return InMap
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
				guard = Guard{map[image.Point]image.Point{{x, y}: {0, -1}}, image.Point{x, y}, image.Point{0, -1}}
			case '#':
				obstructions[image.Point{x, y}] = true
			}
			maxX = x
		}
		y++
	}

	// fmt.Print("\033[H\033[2J")
	for {
		step := stepInArea(obstructions, &guard, maxX, maxY)
		if step == OutOfMap {
			break
		}

		// draw(obstructions, guard, maxX, maxY)
		// time.Sleep(100 * time.Millisecond)
		// fmt.Print("\033[H\033[2J")
	}

	return len(guard.steps)
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var guard Guard
	obstructions := make(map[image.Point]bool)
	var maxX, maxY int
	var start image.Point

	y := 0
	for scanner.Scan() {
		maxY = y
		for x, c := range scanner.Text() {
			switch c {
			case '^':
				guard = Guard{map[image.Point]image.Point{{x, y}: {0, -1}}, image.Point{x, y}, image.Point{0, -1}}
				start = image.Point{x, y}
			case '#':
				obstructions[image.Point{x, y}] = true
			}
			maxX = x
		}
		y++
	}

	for {
		step := stepInArea(obstructions, &guard, maxX, maxY)
		if step == OutOfMap {
			break
		}
	}

	loops := 0
	for step := range guard.steps {
		// "The new obstruction can't be placed at the guard's starting position…"
		if step.Eq(start) {
			continue
		}

		newGuard := Guard{map[image.Point]image.Point{start: {0, -1}}, start, image.Point{0, -1}}
		extendedObstructions := make(map[image.Point]bool)
		for k, v := range obstructions {
			extendedObstructions[k] = v
		}
		extendedObstructions[step] = true
		for {
			step := stepInArea(extendedObstructions, &newGuard, maxX, maxY)
			if step == OutOfMap {
				break
			}
			if step == Loop {
				loops += 1
				break
			}
		}
	}

	return loops
}

func main() {
	fmt.Println("How many distinct positions will the guard visit before leaving the mapped area?", PartOne(input))

	fmt.Println("How many different positions could you choose for this obstruction?", PartTwo(input))
}
