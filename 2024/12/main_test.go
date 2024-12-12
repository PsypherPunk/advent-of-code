package main

import (
	"testing"
)

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		expected int
	}{
		{`AAAA
BBCD
BBCC
EEEC
`, 140},
		{`OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
`, 772},
		{`RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
`, 1930},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartOne(tt.input); actual != tt.expected {
				t.Errorf("PartOne() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}

func Test_PartTwo(t *testing.T) {
	tests := []struct {
		input    string
		expected int
	}{
		{`AAAA
BBCD
BBCC
EEEC
`, 80},
		{`OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
`, 436},
		{`EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
`, 236},
		{`AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
`, 368},
		{`RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
`, 1206},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartTwo(tt.input); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
