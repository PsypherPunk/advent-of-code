package main

import (
	"testing"
)

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{`Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
`, "4,6,3,5,6,3,5,2,1,0"},
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
		{`Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
`, 117440},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartTwo(tt.input); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
