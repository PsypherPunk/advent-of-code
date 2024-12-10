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

func dfsTrailheadScores(topographicMap map[image.Point]rune, position image.Point, seen map[image.Point]bool) (score int) {
	if topographicMap[position] == '9' {
		_, ok := seen[position]

		if ok {
			return 0
		}

		seen[position] = true

		return 1
	}

	for _, step := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
		next := position.Add(step)
		if topographicMap[next] == topographicMap[position]+1 {
			score += dfsTrailheadScores(topographicMap, next, seen)
		}
	}

	return score
}

func dfsDistinctHikingTrails(topographicMap map[image.Point]rune, position image.Point) (score int) {
	if topographicMap[position] == '9' {
		return 1
	}

	for _, step := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
		next := position.Add(step)
		if topographicMap[next] == topographicMap[position]+1 {
			score += dfsDistinctHikingTrails(topographicMap, next)
		}
	}

	return score
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	topographicMap := make(map[image.Point]rune)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			topographicMap[image.Point{x, y}] = r
		}

		y++
	}

	sumOfScores := 0
	for position, height := range topographicMap {
		if height == '0' {
			sumOfScores += dfsTrailheadScores(topographicMap, position, map[image.Point]bool{})
		}
	}

	return sumOfScores
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	topographicMap := make(map[image.Point]rune)

	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			topographicMap[image.Point{x, y}] = r
		}

		y++
	}

	sumOfScores := 0
	for position, height := range topographicMap {
		if height == '0' {
			sumOfScores += dfsDistinctHikingTrails(topographicMap, position)
		}
	}

	return sumOfScores
}

func main() {
	fmt.Println("What is the sum of the scores of all trailheads on your topographic map?", PartOne(input))

	fmt.Println("What is the sum of the ratings of all trailheads?", PartTwo(input))
}
