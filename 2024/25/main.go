package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func fits(key [5]int, lock [5]int) bool {
	for i := 0; i < 5; i++ {
		if key[i]+lock[i] > 5 {
			return false
		}
	}

	return true
}

func PartOne(input string) int {
	schematics := strings.Split(input, "\n\n")

	keys := [][5]int{}
	locks := [][5]int{}

	for _, schematic := range schematics {
		pins := [5]int{}

		lines := []string{}
		scanner := bufio.NewScanner(strings.NewReader(schematic))
		for scanner.Scan() {
			lines = append(lines, scanner.Text())
		}

		if strings.HasPrefix(schematic, "#") {
			for _, line := range lines[1:] {
				for i, r := range line {
					if r == '#' {
						pins[i]++
					}
				}
			}
			locks = append(locks, pins)
		} else {
			for _, line := range lines[:len(lines)-1] {
				for i, r := range line {
					if r == '#' {
						pins[i]++
					}
				}
			}
			keys = append(keys, pins)
		}
	}

	keyPairs := 0
	for _, key := range keys {
		for _, lock := range locks {
			if fits(key, lock) {
				keyPairs++
			}
		}
	}

	return keyPairs
}

func main() {
	fmt.Println("How many unique lock/key pairs fit together without overlapping in any column?", PartOne(input))
}
