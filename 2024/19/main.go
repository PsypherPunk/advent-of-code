package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func isPossible(desiredDesign string, towelPatterns []string) bool {
	possible := make([]bool, len(desiredDesign)+1)
	possible[0] = true

	for i := 1; i <= len(desiredDesign); i++ {
		for _, pattern := range towelPatterns {
			if len(pattern) <= i && possible[i-len(pattern)] && desiredDesign[i-len(pattern):i] == pattern {
				possible[i] = true
				break
			}
		}
	}

	return possible[len(desiredDesign)]
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	towelPatterns := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}

		for _, towelPattern := range strings.Split(line, ", ") {
			towelPatterns = append(towelPatterns, towelPattern)
		}
	}

	desiredDesigns := []string{}
	for scanner.Scan() {
		desiredDesigns = append(desiredDesigns, scanner.Text())
	}

	possibleDesigns := 0
	for _, design := range desiredDesigns {
		if isPossible(design, towelPatterns) {
			possibleDesigns++
		}
	}

	return possibleDesigns
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many designs are possible?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
