package main

import (
	"testing"
)

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		expected int
	}{
		{`190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
`, 3749},
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
	}{}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartTwo(tt.input); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
