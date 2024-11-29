package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

var badStrings = [4]string{"ab", "cd", "pq", "xy"}

func IsNice(line string) bool {
	vowelCount := 0
	hasDouble := false
	hasBadString := false

	for _, c := range line {
		if strings.ContainsRune("aeiou", c) {
			vowelCount += 1
			if vowelCount == 3 {
				break
			}
		}
	}

	for i := 0; i < len(line)-1; i++ {
		if vowelCount != 3 {
			break
		}

		if line[i] == line[i+1] {
			hasDouble = true
			break
		}
	}

	for _, b := range badStrings {
		if vowelCount != 3 || !hasDouble {
			break
		}

		if strings.Contains(line, b) {
			hasBadString = true
			break
		}
	}

	return vowelCount >= 3 && hasDouble && !hasBadString
}

func IsNiceButBetter(line string) bool {
	hasNonOverlappingPair := false
	hasRepeatingLetter := false

	for i := 0; i < len(line)-2; i++ {
		if strings.Contains(line[i+2:], line[i:i+2]) {
			hasNonOverlappingPair = true
			break
		}
	}

	for i := 0; i < len(line)-2; i++ {
		if !hasNonOverlappingPair {
			break
		}

		if line[i] == line[i+2] {
			hasRepeatingLetter = true
			break
		}
	}

	return hasNonOverlappingPair && hasRepeatingLetter
}

func PartOne(input string) int {
	niceCount := 0

	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		if IsNice(line) {
			niceCount++
		}
	}

	return niceCount
}

func PartTwo(input string) int {
	niceCount := 0

	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		if IsNiceButBetter(line) {
			niceCount++
		}
	}

	return niceCount
}

func main() {
	fmt.Println("How many strings are nice?", PartOne(input))

	fmt.Println("How many strings are nice under these new rules?", PartTwo(input))
}
