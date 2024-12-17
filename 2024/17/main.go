package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Registers struct {
	A, B, C int
}

type Instruction struct {
	Opcode, Operand int
}

//go:embed input.txt
var input string

func execute(instruction Instruction, registers *Registers, instructionPointer *int) string {
	var combo int
	var out string

	switch instruction.Operand {
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
		fmt.Println("invalid instruction:", instruction)
	}

	switch instruction.Opcode {
	case 0:
		registers.A = registers.A / int(math.Pow(2, float64(combo)))
		*instructionPointer++
	case 1:
		registers.B = registers.B ^ instruction.Operand
		*instructionPointer++
	case 2:
		registers.B = combo % 8
		*instructionPointer++
	case 3:
		if registers.A != 0 {
			*instructionPointer = instruction.Operand / 2
		} else {
			*instructionPointer++
		}
	case 4:
		registers.B = registers.B ^ registers.C
		*instructionPointer++
	case 5:
		out = strconv.Itoa(combo % 8)
		*instructionPointer++
	case 6:
		registers.B = registers.A / int(math.Pow(2, float64(combo)))
		*instructionPointer++
	case 7:
		registers.C = registers.A / int(math.Pow(2, float64(combo)))
		*instructionPointer++
	}

	return out
}

func PartOne(input string) string {
	registers := Registers{}
	instructions := []Instruction{}

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
			for i := 0; i < len(commaSeparatedInstructions)-1; i += 2 {
				opcode, errOpcode := strconv.Atoi(commaSeparatedInstructions[i])
				operand, errOperand := strconv.Atoi(commaSeparatedInstructions[i+1])
				if errOpcode != nil || errOperand != nil {
					fmt.Println("invalid line:", line, errOpcode, errOperand)
					continue
				}
				instructions = append(instructions, Instruction{opcode, operand})
			}
		}
	}

	instructionPointer := 0
	outputs := []string{}

	for instructionPointer < len(instructions) {
		if out := execute(instructions[instructionPointer], &registers, &instructionPointer); out != "" {
			outputs = append(outputs, out)
		}
	}

	return strings.Join(outputs, ",")
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("…what do you get if you use commas to join the values it output into a single string?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
