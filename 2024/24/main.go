package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func traceWire(wire string, wireValues map[string]int, logicGates map[string]string) int {
	if value, ok := wireValues[wire]; ok {
		return value
	}

	gatesWires, ok := logicGates[wire]
	if !ok {
		fmt.Println("invalid output:", wire)
		return 0
	}
	fields := strings.Fields(gatesWires)
	left, operation, right := fields[0], fields[1], fields[2]
	switch operation {
	case "AND":
		return traceWire(left, wireValues, logicGates) & traceWire(right, wireValues, logicGates)
	case "OR":
		return traceWire(left, wireValues, logicGates) | traceWire(right, wireValues, logicGates)
	case "XOR":
		return traceWire(left, wireValues, logicGates) ^ traceWire(right, wireValues, logicGates)
	default:
		fmt.Println("invalid operation:", operation)
		return 0
	}
}

func PartOne(input string) int64 {
	scanner := bufio.NewScanner(strings.NewReader(input))

	wireValues := make(map[string]int)

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}

		parts := strings.Split(line, ": ")
		value, err := strconv.Atoi(parts[1])
		if err != nil {
			fmt.Println("invalid line:", line)
		}
		wireValues[parts[0]] = value
	}

	logicGates := make(map[string]string)

	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), " -> ")
		logicGates[parts[1]] = parts[0]
	}

	zWires := []string{}
	for k := range wireValues {
		if strings.HasPrefix(k, "z") {
			zWires = append(zWires, k)
		}
	}
	for k := range logicGates {
		if strings.HasPrefix(k, "z") {
			zWires = append(zWires, k)
		}
	}
	sort.Sort(sort.Reverse(sort.StringSlice(zWires)))

	var output strings.Builder
	for _, z := range zWires {
		output.WriteString(strconv.Itoa(traceWire(z, wireValues, logicGates)))
	}

	result, err := strconv.ParseInt(output.String(), 2, 64)
	if err != nil {
		fmt.Println("invalid output:", output)
	}

	return result
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What decimal number does it output on the wires starting with z?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
