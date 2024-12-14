package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"strings"
)

//go:embed input.txt
var input string

type Robot struct {
	P image.Point
	V image.Point
}

const (
	Width  = 101
	Height = 103
	// Width  = 11
	// Height = 7
)

func (robot *Robot) runSeconds(seconds int) {
	robot.P.X = robot.P.X + seconds*robot.V.X
	robot.P.Y = robot.P.Y + seconds*robot.V.Y
	if robot.P.X < 0 {
		n := (-robot.P.X / Width) + 1
		robot.P.X += n * Width
	}
	if robot.P.Y < 0 {
		n := (-robot.P.Y / Height) + 1
		robot.P.Y += n * Height
	}
	robot.P.X %= Width
	robot.P.Y %= Height
	robot.P.X = robot.P.X
	robot.P.Y = robot.P.Y
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	robots := []Robot{}

	for scanner.Scan() {
		robot := Robot{}
		fmt.Sscanf(scanner.Text(), "p=%d,%d v=%d,%d", &robot.P.X, &robot.P.Y, &robot.V.X, &robot.V.Y)
		robot.runSeconds(100)
		robots = append(robots, robot)
	}

	var q1, q2, q3, q4 int
	for _, robot := range robots {
		if robot.P.X < (Width-1)/2 && robot.P.Y < (Height-1)/2 {
			q1++
		}
		if robot.P.X < (Width-1)/2 && robot.P.Y > (Height-1)/2 {
			q2++
		}
		if robot.P.X > (Width-1)/2 && robot.P.Y < (Height-1)/2 {
			q3++
		}
		if robot.P.X > (Width-1)/2 && robot.P.Y > (Height-1)/2 {
			q4++
		}
	}

	return q1 * q2 * q3 * q4
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What will the safety factor be after exactly 100 seconds have elapsed?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
