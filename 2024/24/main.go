package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type logicGate struct {
	operation   string
	left        string
	right       string
	destination string
}

var xyz = []string{"x", "y", "z"}

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

func hasAnyPrefix(s string, prefixes []string) bool {
	for _, prefix := range prefixes {
		if strings.HasPrefix(s, prefix) {
			return true
		}
	}

	return false
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

func PartTwo(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	wireValues := make(map[string]int)
	logicGates := make(map[logicGate]struct{})

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

	for scanner.Scan() {
		var gate logicGate

		fmt.Sscanf(scanner.Text(), "%s %s %s -> %s", &gate.left, &gate.operation, &gate.right, &gate.destination)
		logicGates[gate] = struct{}{}
	}

	swapped := make(map[string]struct{})

	for gate := range logicGates {
		if strings.HasPrefix(gate.destination, "z") && gate.operation != "XOR" && gate.destination != "z45" {
			swapped[gate.destination] = struct{}{}
			continue
		}

		if gate.operation == "XOR" &&
			!hasAnyPrefix(gate.destination, xyz) &&
			!hasAnyPrefix(gate.left, xyz) &&
			!hasAnyPrefix(gate.right, xyz) {
			swapped[gate.destination] = struct{}{}
			continue
		}

		if gate.operation == "AND" && (gate.left != "x00" && gate.right != "x00") {
			for other := range logicGates {
				if (gate.destination == other.left || gate.destination == other.right) && other.operation != "OR" {
					swapped[gate.destination] = struct{}{}
					break
				}
			}
		}

		if gate.operation == "XOR" {
			for other := range logicGates {
				if (gate.destination == other.left || gate.destination == other.right) && other.operation == "OR" {
					swapped[gate.destination] = struct{}{}
					break
				}
			}
		}
	}

	wires := make([]string, 0, len(swapped))
	for wire := range swapped {
		wires = append(wires, wire)
	}

	sort.Strings(wires)

	return strings.Join(wires, ",")
}

func main() {
	fmt.Println("What decimal number does it output on the wires starting with z?", PartOne(input))

	fmt.Println("…what do you get if you sort the names of the eight wires involved in a swap and then join those names with commas?", PartTwo(input))
}
