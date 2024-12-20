package main

import (
	"bufio"
	"container/list"
	_ "embed"
	"fmt"
	"image"
	"strings"
)

//go:embed input.txt
var input string

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func PartOne(input string, saving int) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	racetrack := make(map[image.Point]bool)
	start := image.Point{}
	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			switch r {
			case 'S':
				start = image.Point{x, y}
				racetrack[image.Point{x, y}] = true
			case '.', 'E':
				racetrack[image.Point{x, y}] = true
			}
		}

		y++
	}

	queue := list.New()
	queue.PushBack(start)
	distances := map[image.Point]int{start: 0}

	for queue.Len() > 0 {
		current := queue.Remove(queue.Front()).(image.Point)

		for _, direction := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
			next := current.Add(direction)
			if _, ok := distances[next]; !ok && racetrack[next] {
				queue.PushBack(next)
				distances[next] = distances[current] + 1
			}
		}
	}

	cheats := 0
	for a := range distances {
		for b := range distances {
			distance := abs(b.X-a.X) + abs(b.Y-a.Y)
			if distance <= 2 && distances[b] >= distances[a]+distance+saving {
				cheats++
			}
		}
	}

	return cheats
}

func PartTwo(input string, saving int) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	racetrack := make(map[image.Point]bool)
	start := image.Point{}
	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			switch r {
			case 'S':
				start = image.Point{x, y}
				racetrack[image.Point{x, y}] = true
			case '.', 'E':
				racetrack[image.Point{x, y}] = true
			}
		}

		y++
	}

	queue := list.New()
	queue.PushBack(start)
	distances := map[image.Point]int{start: 0}

	for queue.Len() > 0 {
		current := queue.Remove(queue.Front()).(image.Point)

		for _, direction := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
			next := current.Add(direction)
			if _, ok := distances[next]; !ok && racetrack[next] {
				queue.PushBack(next)
				distances[next] = distances[current] + 1
			}
		}
	}

	cheats := 0
	for a := range distances {
		for b := range distances {
			distance := abs(b.X-a.X) + abs(b.Y-a.Y)
			if distance <= 20 && distances[b] >= distances[a]+distance+saving {
				cheats++
			}
		}
	}

	return cheats
}

func main() {
	fmt.Println("How many cheats would save you at least 100 picoseconds?", PartOne(input, 100))

	fmt.Println("How many cheats would save you at least 100 picoseconds?", PartTwo(input, 100))
}
