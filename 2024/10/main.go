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

func dfs(topographicMap map[image.Point]rune, position image.Point, seen map[image.Point]bool) (score int) {
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
			score += dfs(topographicMap, next, seen)
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
			sumOfScores += dfs(topographicMap, position, map[image.Point]bool{})
		}
	}

	return sumOfScores
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What is the sum of the scores of all trailheads on your topographic map?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
