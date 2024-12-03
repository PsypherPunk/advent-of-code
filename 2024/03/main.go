package main

import (
	_ "embed"
	"fmt"
	"regexp"
	"strconv"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	sum := 0
	re := regexp.MustCompile(`mul\((\d+),(\d+)\)`)

	matches := re.FindAllStringSubmatch(input, -1)
	for _, match := range matches {
		a, errA := strconv.Atoi(match[1])
		b, errB := strconv.Atoi(match[2])

		if errA != nil || errB != nil {
			fmt.Println("Invalid match:", match)
			continue
		}

		sum += a * b
	}

	return sum
}

func PartTwo(input string) int {
	sum := 0
	re := regexp.MustCompile(`(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))`)
	enabled := true

	matches := re.FindAllStringSubmatch(input, -1)
	for _, match := range matches {
		switch prefix := match[0][:3]; prefix {
		case "mul":
			if !enabled {
				continue
			}

			a, errA := strconv.Atoi(match[1])
			b, errB := strconv.Atoi(match[2])
			if errA != nil || errB != nil {
				fmt.Println("invalid mul():", match)
				continue
			}

			sum += a * b
		case "don":
			enabled = false
		case "do(":
			enabled = true
		default:
			fmt.Println("invalid match:", prefix)
		}
	}

	return sum
}

func main() {
	fmt.Println("What do you get if you add up all of the results of the multiplications?", PartOne(input))

	fmt.Println("…what do you get if you add up all of the results of just the enabled multiplications?", PartTwo(input))
}
