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

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	garden := make(map[image.Point]rune)
	notSeen := make(map[image.Point]bool)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			garden[image.Point{x, y}] = r
			notSeen[image.Point{x, y}] = true
		}
		y++
	}

	directions := []image.Point{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	price := 0
	for len(notSeen) > 0 {
		var start image.Point
		for k := range notSeen {
			start = k
			break
		}
		delete(notSeen, start)
		plot := garden[start]

		stack := []image.Point{start}
		area, perimeter := 0, 0
		sides := make(map[[2]image.Point]bool)

		for len(stack) > 0 {
			currentPlot := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			area++
			perimeter += 4

			for _, direction := range directions {
				neighbourPlot := currentPlot.Add(direction)
				if garden[neighbourPlot] != plot {
					sides[[2]image.Point{currentPlot, direction}] = true
					continue
				}
				perimeter--
				if !notSeen[neighbourPlot] {
					continue
				}
				delete(notSeen, neighbourPlot)
				stack = append(stack, neighbourPlot)
			}
		}

		price += area * perimeter
	}

	return price
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	garden := make(map[image.Point]rune)
	notSeen := make(map[image.Point]bool)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			garden[image.Point{x, y}] = r
			notSeen[image.Point{x, y}] = true
		}
		y++
	}

	directions := []image.Point{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	price := 0
	for len(notSeen) > 0 {
		var start image.Point
		for k := range notSeen {
			start = k
			break
		}
		delete(notSeen, start)
		plot := garden[start]

		stack := []image.Point{start}
		area, perimeter := 0, 0
		sides := make(map[[2]image.Point]bool)

		for len(stack) > 0 {
			currentPlot := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			area++
			perimeter += 4

			for _, direction := range directions {
				neighbourPlot := currentPlot.Add(direction)
				if garden[neighbourPlot] != plot {
					sides[[2]image.Point{currentPlot, direction}] = true
					continue
				}
				perimeter--
				if !notSeen[neighbourPlot] {
					continue
				}
				delete(notSeen, neighbourPlot)
				stack = append(stack, neighbourPlot)
			}
		}

		adjustment := 0
		for side := range sides {
			plot, direction := side[0], side[1]
			if sides[[2]image.Point{plot.Add(image.Point{X: direction.Y, Y: -direction.X}), direction}] {
				adjustment--
			}
		}

		price += area * (len(sides) + adjustment)
	}

	return price
}

func main() {
	fmt.Println("What is the total price of fencing all regions on your map?", PartOne(input))

	fmt.Println("What is the new total price of fencing all regions on your map?", PartTwo(input))
}
