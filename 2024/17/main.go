package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

type Registers struct {
	A, B, C int
}

//go:embed input.txt
var input string

func execute(instructions []int, registers Registers) (outputs []int) {
	var combo int
	instructionPointer := 0

	for instructionPointer < len(instructions)-1 {

		switch instructions[instructionPointer+1] {
		case 0:
			combo = 0
		case 1:
			combo = 1
		case 2:
			combo = 2
		case 3:
			combo = 3
		case 4:
			combo = registers.A
		case 5:
			combo = registers.B
		case 6:
			combo = registers.C
		default:
			fmt.Println("invalid instruction:", instructions, instructionPointer)
		}

		switch instructions[instructionPointer] {
		case 0:
			registers.A = registers.A / int(math.Pow(2, float64(combo)))
			instructionPointer += 2
		case 1:
			registers.B = registers.B ^ instructions[instructionPointer+1]
			instructionPointer += 2
		case 2:
			registers.B = combo % 8
			instructionPointer += 2
		case 3:
			if registers.A != 0 {
				instructionPointer = instructions[instructionPointer+1] / 2
			} else {
				instructionPointer += 2
			}
		case 4:
			registers.B = registers.B ^ registers.C
			instructionPointer += 2
		case 5:
			outputs = append(outputs, combo%8)
			instructionPointer += 2
		case 6:
			registers.B = registers.A / int(math.Pow(2, float64(combo)))
			instructionPointer += 2
		case 7:
			registers.C = registers.A / int(math.Pow(2, float64(combo)))
			instructionPointer += 2
		}
	}

	return outputs
}

func PartOne(input string) string {
	registers := Registers{}
	instructions := []int{}

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}

		switch line[9] {
		case 'A':
			fmt.Sscanf(line, "Register A: %d", &registers.A)
		case 'B':
			fmt.Sscanf(line, "Register A: %d", &registers.B)
		case 'C':
			fmt.Sscanf(line, "Register A: %d", &registers.C)
		default:
			commaSeparatedInstructions := strings.Split(line[9:], ",")
			for i := 0; i < len(commaSeparatedInstructions); i++ {
				intruction, err := strconv.Atoi(commaSeparatedInstructions[i])
				if err != nil {
					fmt.Println("invalid line:", line, err)
					continue
				}
				instructions = append(instructions, intruction)
			}
		}
	}

	outputs := execute(instructions, registers)
	result := []string{}
	for _, output := range outputs {
		result = append(result, strconv.Itoa(output))
	}

	return strings.Join(result, ",")
}

func PartTwo(input string) int {
	registers := Registers{}
	instructions := []int{}

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}

		switch line[9] {
		case 'A':
			fmt.Sscanf(line, "Register A: %d", &registers.A)
		case 'B':
			fmt.Sscanf(line, "Register A: %d", &registers.B)
		case 'C':
			fmt.Sscanf(line, "Register A: %d", &registers.C)
		default:
			commaSeparatedInstructions := strings.Split(line[9:], ",")
			for i := 0; i < len(commaSeparatedInstructions); i++ {
				intruction, err := strconv.Atoi(commaSeparatedInstructions[i])
				if err != nil {
					fmt.Println("invalid line:", line, err)
					continue
				}
				instructions = append(instructions, intruction)
			}
		}
	}

	registers = Registers{0, 0, 0}
	for instructionPointer := len(instructions) - 1; instructionPointer >= 0; instructionPointer-- {
		registers.A <<= 3
		for !slices.Equal(execute(instructions, registers), instructions[instructionPointer:]) {
			registers.A++
		}
	}

	return registers.A
}

func main() {
	fmt.Println("…what do you get if you use commas to join the values it output into a single string?", PartOne(input))

	fmt.Println("What is the lowest positive initial value for register A that causes the program to output a copy of itself?", PartTwo(input))
}
