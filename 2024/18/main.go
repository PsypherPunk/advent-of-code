package main

import (
	"bufio"
	"container/list"
	_ "embed"
	"fmt"
	"image"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

type Path struct {
	bytePosition image.Point
	steps        int
}

func reachExit(fallingBytes []image.Point, max int) int {
	topLeft := image.Point{0, 0}
	bottomRight := image.Point{max, max}

	queue := list.New()
	queue.PushBack(Path{topLeft, 0})

	seen := make(map[image.Point]bool)
	seen[topLeft] = true

	fallenBytes := map[image.Point]bool{}
	for _, fallingByte := range fallingBytes {
		fallenBytes[fallingByte] = true
	}

	for queue.Len() > 0 {
		current := queue.Remove(queue.Front()).(Path)

		if current.bytePosition.Eq(bottomRight) {
			return current.steps
		}

		for _, direction := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
			next := current.bytePosition.Add(direction)
			if next.X >= 0 && next.X <= 70 && next.Y >= 0 && next.Y <= 70 && !fallenBytes[next] && !seen[next] {
				seen[next] = true
				queue.PushBack(Path{next, current.steps + 1})
			}
		}
	}

	return -1
}

func PartOne(input string, fallen int, max int) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	bytePositions := []image.Point{}
	for scanner.Scan() {
		line := scanner.Text()

		bytePosition := strings.Split(line, ",")
		byteX, errX := strconv.Atoi(bytePosition[0])
		byteY, errY := strconv.Atoi(bytePosition[1])

		if errX != nil || errY != nil {
			fmt.Println("invalid line:", line, errX, errY)
			continue
		}
		bytePositions = append(bytePositions, image.Point{byteX, byteY})
	}

	return reachExit(bytePositions[:fallen], max)
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("…what is the minimum number of steps needed to reach the exit?", PartOne(input, 1024, 70))

	fmt.Println("", PartTwo(input))
}
