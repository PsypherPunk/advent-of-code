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
	scanner := bufio.NewScanner(strings.NewReader(input))

	var aX, aY, bX, bY, pX, pY int

	totalTokens := 0
	for scanner.Scan() {
		line := scanner.Text()

		if len(line) == 0 {
			continue
		}

		switch line[:8] {
		case "Button A":
			fmt.Sscanf(line, "Button A: X+%d, Y+%d", &aX, &aY)
		case "Button B":
			fmt.Sscanf(line, "Button B: X+%d, Y+%d", &bX, &bY)
		case "Prize: X":
			fmt.Sscanf(line, "Prize: X=%d, Y=%d", &pX, &pY)

			// https://en.wikipedia.org/wiki/Cramer%27s_rule
			D, Dx, Dy := aX*bY-bX*aY, pX*bY-bX*pY, aX*pY-pX*aY
			if D != 0 && Dx == (Dx/D)*D && Dy == (Dy/D)*D {
				totalTokens += (Dx/D)*3 + (Dy / D)
			}

			aX, aY, bX, bY, pX, pY = 0, 0, 0, 0, 0, 0
		}
	}

	return totalTokens
}

func PartTwo(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var aX, aY, bX, bY, pX, pY int

	totalTokens := 0
	for scanner.Scan() {
		line := scanner.Text()

		if len(line) == 0 {
			continue
		}

		switch line[:8] {
		case "Button A":
			fmt.Sscanf(line, "Button A: X+%d, Y+%d", &aX, &aY)
		case "Button B":
			fmt.Sscanf(line, "Button B: X+%d, Y+%d", &bX, &bY)
		case "Prize: X":
			fmt.Sscanf(line, "Prize: X=%d, Y=%d", &pX, &pY)
			pX, pY = pX+10000000000000, pY+10000000000000

			// https://en.wikipedia.org/wiki/Cramer%27s_rule
			D, Dx, Dy := aX*bY-bX*aY, pX*bY-bX*pY, aX*pY-pX*aY
			if D != 0 && Dx == (Dx/D)*D && Dy == (Dy/D)*D {
				totalTokens += (Dx/D)*3 + (Dy / D)
			}

			aX, aY, bX, bY, pX, pY = 0, 0, 0, 0, 0, 0
		}
	}

	return totalTokens
}

func main() {
	fmt.Println("What is the fewest tokens you would have to spend to win all possible prizes?", PartOne(input))

	fmt.Println("What is the fewest tokens you would have to spend to win all possible prizes?", PartTwo(input))
}
