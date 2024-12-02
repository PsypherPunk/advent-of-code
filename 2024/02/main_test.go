package main

import (
	"testing"
)

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		expected bool
	}{
		{"7 6 4 2 1", true},
		{"1 2 7 8 9", false},
		{"9 7 6 2 1", false},
		{"1 3 2 4 5", false},
		{"8 6 4 4 1", false},
		{"1 3 6 7 9", true},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := IsSafe(tt.input); actual != tt.expected {
				t.Errorf("PartOne() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}

func Test_PartTwo(t *testing.T) {
	tests := []struct {
		input    string
		expected bool
	}{}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := IsSafeButTolerant(tt.input); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
