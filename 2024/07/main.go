package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func IsValid(testValue int, numbers []int, i int, currentValue int) bool {
	if i >= len(numbers) {
		return testValue == currentValue
	}

	if currentValue > testValue {
		return false
	}

	return IsValid(testValue, numbers, i+1, currentValue+numbers[i]) || IsValid(testValue, numbers, i+1, currentValue*numbers[i])
}

func IsStillValid(testValue int, numbers []int, i int, currentValue int) bool {
	if i >= len(numbers) {
		return testValue == currentValue
	}

	if currentValue > testValue {
		return false
	}

	nextValue, err := strconv.Atoi(fmt.Sprintf("%d%d", currentValue, numbers[i]))
	if err != nil {
		fmt.Println("invalid numbers", numbers)
		return false
	}

	return IsStillValid(testValue, numbers, i+1, currentValue+numbers[i]) ||
		IsStillValid(testValue, numbers, i+1, currentValue*numbers[i]) ||
		IsStillValid(testValue, numbers, i+1, nextValue)
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	totalCalibrationResult := 0
	for scanner.Scan() {
		line := scanner.Text()

		testValueNumbers := strings.SplitN(line, ":", 2)
		testValue, err := strconv.Atoi(testValueNumbers[0])
		if err != nil {
			fmt.Println("invalid line:", line, err)
		}

		stringNumbers := strings.Fields(testValueNumbers[1])
		numbers := make([]int, len(stringNumbers), len(stringNumbers))
		for i, stringNumber := range stringNumbers {
			number, err := strconv.Atoi(stringNumber)
			if err != nil {
				fmt.Println("invalid line", line, err)
			}
			numbers[i] = number
		}

		if IsValid(testValue, numbers, 1, numbers[0]) {
			totalCalibrationResult += testValue
		}
	}

	return totalCalibrationResult
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	totalCalibrationResult := 0
	for scanner.Scan() {
		line := scanner.Text()

		testValueNumbers := strings.SplitN(line, ":", 2)
		testValue, err := strconv.Atoi(testValueNumbers[0])
		if err != nil {
			fmt.Println("invalid line:", line, err)
		}

		stringNumbers := strings.Fields(testValueNumbers[1])
		numbers := make([]int, len(stringNumbers), len(stringNumbers))
		for i, stringNumber := range stringNumbers {
			number, err := strconv.Atoi(stringNumber)
			if err != nil {
				fmt.Println("invalid line", line, err)
			}
			numbers[i] = number
		}

		if IsStillValid(testValue, numbers, 1, numbers[0]) {
			totalCalibrationResult += testValue
		}
	}

	return totalCalibrationResult
}

func main() {
	fmt.Println("What is their total calibration result?", PartOne(input))

	fmt.Println("What is their total calibration result?", PartTwo(input))
}
