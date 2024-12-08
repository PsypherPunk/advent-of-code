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

func applyNefariousEffects(a, b image.Point, maxX, maxY int) []image.Point {
	antinodes := []image.Point{}

	ax := 2*b.X - a.X
	ay := 2*b.Y - a.Y
	if ay >= 0 && ay <= maxY && ax >= 0 && ax <= maxX {
		antinodes = append(antinodes, image.Point{ax, ay})
	}

	ax = 2*a.X - b.X
	ay = 2*a.Y - b.Y
	if ay >= 0 && ay <= maxY && ax >= 0 && ax <= maxX {
		antinodes = append(antinodes, image.Point{ax, ay})
	}

	return antinodes
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var maxX, maxY int
	city := make(map[rune]map[image.Point]bool)

	y := 0
	for scanner.Scan() {
		maxY = y
		for x, r := range scanner.Text() {
			if r != '.' {
				_, ok := city[r]
				if !ok {
					city[r] = make(map[image.Point]bool)
				}
				city[r][image.Point{x, y}] = true
			}
			maxX = x
		}
		y++
	}

	antinodes := make(map[image.Point]bool)
	for _, antennae := range city {
		points := []image.Point{}

		for point := range antennae {
			points = append(points, point)
		}

		for i, a := range points {
			for _, b := range points[i+1:] {
				for _, antinode := range applyNefariousEffects(a, b, maxX, maxY) {
					antinodes[antinode] = true
				}
			}
		}
	}

	return len(antinodes)
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many unique locations within the bounds of the map contain an antinode?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
