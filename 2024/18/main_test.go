package main

import (
	"testing"
)

func Test_PartOne(t *testing.T) {
	tests := []struct {
		input    string
		count    int
		max      int
		expected int
	}{
		{`5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
`, 12, 6, 22},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartOne(tt.input, tt.count, tt.max); actual != tt.expected {
				t.Errorf("PartOne() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}

func Test_PartTwo(t *testing.T) {
	tests := []struct {
		input    string
		max      int
		expected string
	}{
		{`5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
`, 6, "6,1"},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			if actual := PartTwo(tt.input, tt.max); actual != tt.expected {
				t.Errorf("PartTwo() = %v, expected %v", actual, tt.expected)
			}
		})
	}
}
