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

func wordsFromPoint(wordSearch map[image.Point]rune, point image.Point, count int) []string {
	// Specifically orient the corners first…
	directions := []image.Point{
		{-1, -1},
		{1, -1},
		{1, 1},
		{-1, 1},
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	words := make([]string, len(directions))
	for i, p := range directions {
		for n := range count {
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
		for _, word := range wordsFromPoint(wordSearch, p, 4) {
			if word == "XMAS" {
				found++
			}
		}
	}

	return found
}

// Possible orientations are:
//
// M.S
// .A.
// M.S
//
// M.M
// .A.
// S.S
//
// S.M
// .A.
// S.M
//
// S.S
// .A.
// M.M
//
// …which can be encoded as a cyclic substring:
//
// ..AMASASAM
// AMAMASAS
// ......ASAMAMAS
// ....ASASAMAM
//
// AMAMASASAMAMAS
func PartTwo(input string) int {
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
		words := wordsFromPoint(wordSearch, p, 2)

		found += strings.Count("AMAMASASAMAMAS", strings.Join(words[:4], ""))
	}

	return found
}

func main() {
	fmt.Println("How many times does XMAS appear?", PartOne(input))

	fmt.Println("How many times does an X-MAS appear?", PartTwo(input))
}
