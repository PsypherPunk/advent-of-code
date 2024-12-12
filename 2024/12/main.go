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

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	garden := make(map[image.Point]rune)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			garden[image.Point{x, y}] = r
		}
		y++
	}

	directions := []image.Point{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	seen := make(map[image.Point]bool)
	price := 0

	for plot, plant := range garden {
		if seen[plot] {
			continue
		}
		seen[plot] = true

		area, perimeter := 0, 0
		stack := list.New()
		stack.PushBack(plot)

		for stack.Len() > 0 {
			current := stack.Remove(stack.Back()).(image.Point)
			area++
			perimeter += 4

			for _, direction := range directions {
				neighbourPlot := current.Add(direction)

				if garden[neighbourPlot] != plant {
					continue
				}
				perimeter--
				if seen[neighbourPlot] {
					continue
				}
				seen[neighbourPlot] = true
				stack.PushBack(neighbourPlot)
			}
		}

		price += area * perimeter
	}

	return price
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	garden := make(map[image.Point]rune)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			garden[image.Point{x, y}] = r
		}
		y++
	}

	directions := []image.Point{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	seen := make(map[image.Point]bool)
	price := 0

	for plot, plant := range garden {
		if seen[plot] {
			continue
		}
		seen[plot] = true

		area, perimeter := 0, 0
		sides := make(map[[2]image.Point]bool)
		stack := list.New()
		stack.PushBack(plot)

		for stack.Len() > 0 {
			current := stack.Remove(stack.Back()).(image.Point)
			area++
			perimeter += 4

			for _, direction := range directions {
				neighbourPlot := current.Add(direction)

				if garden[neighbourPlot] != plant {
					sides[[2]image.Point{current, direction}] = true
					continue
				}
				perimeter--
				if seen[neighbourPlot] {
					continue
				}
				seen[neighbourPlot] = true
				stack.PushBack(neighbourPlot)
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
