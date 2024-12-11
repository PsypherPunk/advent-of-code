package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func Blink(stones map[int]int, blinks int) (totalStones int) {
	for range blinks {
		next := map[int]int{}
		for engraving, count := range stones {
			if engraving == 0 {
				next[1] += count
			} else if s := strconv.Itoa(engraving); len(s)%2 == 0 {
				left, errLeft := strconv.Atoi(s[:len(s)/2])
				right, errRight := strconv.Atoi(s[len(s)/2:])
				if errLeft != nil || errRight != nil {
					fmt.Println("invalid stones: ", engraving)
					continue
				}
				next[left] += count
				next[right] += count
			} else {
				next[engraving*2024] += count
			}
		}
		stones = next
	}

	for _, count := range stones {
		totalStones += count
	}

	return totalStones
}

func PartOne(input string) int {
	fields := strings.Fields(input)

	stones := map[int]int{}
	for _, f := range fields {
		stone, err := strconv.Atoi(f)
		if err != nil {
			fmt.Println("invalid input: ", input)
			continue
		}
		stones[stone]++
	}

	return Blink(stones, 25)
}

func PartTwo(input string) int {
	fields := strings.Fields(input)

	stones := map[int]int{}
	for _, f := range fields {
		stone, err := strconv.Atoi(f)
		if err != nil {
			fmt.Println("invalid input: ", input)
			continue
		}
		stones[stone]++
	}

	return Blink(stones, 75)
}

func main() {
	fmt.Println("How many stones will you have after blinking 25 times?", PartOne(input))

	fmt.Println("How many stones would you have after blinking a total of 75 times?", PartTwo(input))
}
