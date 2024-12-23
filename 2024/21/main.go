package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"math"
	"strconv"
	"strings"
)

var numericKeypad = map[rune]image.Point{
	'7': {0, 0},
	'8': {1, 0},
	'9': {2, 0},
	'4': {0, 1},
	'5': {1, 1},
	'6': {2, 1},
	'1': {0, 2},
	'2': {1, 2},
	'3': {2, 2},
	'0': {1, 3},
	'A': {2, 3},
}

var directionalKeypad = map[rune]image.Point{
	'^': {1, 0},
	'A': {2, 0},
	'<': {0, 1},
	'v': {1, 1},
	'>': {2, 1},
}

var directions = map[rune]image.Point{
	'^': {0, -1},
	'v': {0, 1},
	'>': {1, 0},
	'<': {-1, 0},
}

var (
	inverseDirectionalKeypad = inverseKeypad(directionalKeypad)
	inverseNumericKeypad     = inverseKeypad(numericKeypad)
)

var (
	shortestSequenceCache map[string]int
	sequencesCache        map[string][]string
)

//go:embed input.txt
var input string

func inverseKeypad(keypad map[rune]image.Point) map[image.Point]rune {
	inverse := make(map[image.Point]rune)

	for k, v := range keypad {
		inverse[v] = k
	}

	return inverse
}

func complexities(codes []string, robots int) int {
	complexity := 0

	for _, code := range codes {
		shortestSequence := shortestSequence("A"+code, robots)
		numericPart, err := strconv.Atoi(code[:len(code)-1])
		if err != nil {
			fmt.Println("invalid code:", code)
			continue
		}
		complexity += shortestSequence * numericPart
	}

	return complexity
}

func shortestSequence(buttons string, robots int) int {
	presses := 0

	for i := 0; i < len(buttons)-1; i++ {
		currentSequence := getShortestSequence([]rune(buttons)[i], []rune(buttons)[i+1], numericKeypad, inverseNumericKeypad, robots)
		presses += currentSequence
	}

	return presses
}

func getShortestSequence(a, b rune, keypad map[rune]image.Point, inverseKeypad map[image.Point]rune, robots int) int {
	key := fmt.Sprintf("%c%c%d", a, b, robots)

	if shortestSequence, ok := shortestSequenceCache[key]; ok {
		return shortestSequence
	}

	if robots == 0 {
		minLen := math.MaxInt
		for _, sequence := range allSequences(a, b, directionalKeypad, inverseDirectionalKeypad) {
			minLen = min(minLen, len(sequence))
		}
		return minLen
	}

	sequences := allSequences(a, b, keypad, inverseKeypad)
	minSequence := math.MaxInt

	for _, sequence := range sequences {
		sequence = "A" + sequence
		var currentSequence int

		for i := 0; i < len(sequence)-1; i++ {
			currentSequence += getShortestSequence([]rune(sequence)[i], []rune(sequence)[i+1], directionalKeypad, inverseDirectionalKeypad, robots-1)
		}
		minSequence = min(minSequence, currentSequence)
	}

	shortestSequenceCache[key] = minSequence

	return minSequence
}

func allSequences(a, b rune, keypad map[rune]image.Point, inverseKeypad map[image.Point]rune) (allSequences []string) {
	key := fmt.Sprintf("%c %c", a, b)

	if sequences, ok := sequencesCache[key]; ok {
		return sequences
	}

	dfs(keypad[a], keypad[b], []rune{}, keypad, inverseKeypad, make(map[image.Point]bool), &allSequences)

	sequencesCache[key] = allSequences

	return
}

func dfs(current, end image.Point, sequence []rune, keypad map[rune]image.Point, inverseKeypad map[image.Point]rune, seen map[image.Point]bool, allSequences *[]string) {
	if current == end {
		*allSequences = append(*allSequences, string(sequence)+"A")
		return
	}

	seen[current] = true
	for r, direction := range directions {
		next := current.Add(direction)
		if _, ok := inverseKeypad[next]; ok && !seen[next] {
			dfs(next, end, append(sequence, r), keypad, inverseKeypad, seen, allSequences)
		}
	}
	seen[current] = false
}

func PartOne(input string) int {
	shortestSequenceCache = make(map[string]int)
	sequencesCache = make(map[string][]string)

	scanner := bufio.NewScanner(strings.NewReader(input))
	codes := []string{}
	for scanner.Scan() {
		codes = append(codes, scanner.Text())
	}

	return complexities(codes, 2)
}

func PartTwo(input string) int {
	shortestSequenceCache = make(map[string]int)
	sequencesCache = make(map[string][]string)

	scanner := bufio.NewScanner(strings.NewReader(input))
	codes := []string{}
	for scanner.Scan() {
		codes = append(codes, scanner.Text())
	}

	return complexities(codes, 25)
}

func main() {
	fmt.Println("What is the sum of the complexities of the five codes on your list?", PartOne(input))

	fmt.Println("What is the sum of the complexities of the five codes on your list?", PartTwo(input))
}
