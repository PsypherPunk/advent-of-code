package main

import (
	"testing"
)

const racetrack = `###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
`

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		saving   int
		expected int
	}{
		{racetrack, 64, 1},
		{racetrack, 40, 2},
		{racetrack, 38, 3},
		{racetrack, 36, 4},
		{racetrack, 20, 5},
		{racetrack, 12, 8},
		{racetrack, 10, 10},
		{racetrack, 8, 14},
		{racetrack, 6, 16},
		{racetrack, 4, 30},
		{racetrack, 2, 44},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartOne(tt.input, tt.saving); actual != tt.expected {
				t.Errorf("PartOne() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}

func Test_PartTwo(t *testing.T) {
	tests := []struct {
		input    string
		saving   int
		expected int
	}{
		{racetrack, 76, 3},
		{racetrack, 74, 7},
		{racetrack, 72, 29},
		{racetrack, 70, 41},
		{racetrack, 68, 55},
		{racetrack, 66, 67},
		{racetrack, 64, 86},
		{racetrack, 62, 106},
		{racetrack, 60, 129},
		{racetrack, 58, 154},
		{racetrack, 56, 193},
		{racetrack, 54, 222},
		{racetrack, 52, 253},
		{racetrack, 50, 285},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartTwo(tt.input, tt.saving); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
