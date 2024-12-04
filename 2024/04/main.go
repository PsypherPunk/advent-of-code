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

func wordsFromPoint(wordSearch map[image.Point]rune, point image.Point) []string {
	directions := []image.Point{
		{0, -1},
		{1, -1},
		{1, 0},
		{1, 1},
		{0, 1},
		{-1, 1},
		{-1, 0},
		{-1, -1},
	}

	words := make([]string, len(directions))
	for i, p := range directions {
		for n := range 4 {
			words[i] += string(wordSearch[point.Add(p.Mul(n))])
		}
	}
	return words
}

func PartOne(input string) int {
	wordSearch := map[image.Point]rune{}

	scanner := bufio.NewScanner(strings.NewReader(input))
	y := 0
	for scanner.Scan() {
		line := scanner.Text()

		for x, r := range line {
			wordSearch[image.Point{x, y}] = r
		}

		y++
	}

	found := 0
	for p := range wordSearch {
		for _, word := range wordsFromPoint(wordSearch, p) {
			if word == "XMAS" {
				found++
			}
		}
	}

	return found
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many times does XMAS appear?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
