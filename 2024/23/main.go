package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	adjacencyList := make(map[string]map[string]bool)

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "-")
		a, b := parts[0], parts[1]
		if adjacencyList[a] == nil {
			adjacencyList[a] = make(map[string]bool)
		}
		if adjacencyList[b] == nil {
			adjacencyList[b] = make(map[string]bool)
		}
		adjacencyList[a][b] = true
		adjacencyList[b][a] = true
	}

	interConnected := [][]string{}
	for a := range adjacencyList {
		for b := range adjacencyList[a] {
			if b <= a {
				continue
			}
			for c := range adjacencyList[b] {
				if c <= b {
					continue
				}
				if !adjacencyList[a][c] {
					continue
				}
				interConnected = append(interConnected, []string{a, b, c})
			}
		}
	}

	count := 0
	for _, setOfThree := range interConnected {
		for _, computer := range setOfThree {
			if strings.HasPrefix(computer, "t") {
				count++
				break
			}
		}
	}

	return count
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("How many contain at least one computer with a name that starts with t?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
