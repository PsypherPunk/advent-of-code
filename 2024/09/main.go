package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func getLeftmostFreeSpaceBlock(blocks []int) int {
	for i, id := range blocks {
		if id == -1 {
			return i
		}
	}

	return -1
}

func getEndOfDiskBlock(blocks []int) int {
	for i := len(blocks) - 1; i >= 0; i-- {
		if blocks[i] != -1 {
			return i
		}
	}

	return -1
}

func checksum(blocks []int) int {
	checksum := 0

	for i, id := range blocks {
		if id == -1 {
			break
		}
		checksum += i * id
	}

	return checksum
}

func PartOne(input string) int {
	blocks := []int{}

	fileId := 0
	for i, digit := range strings.TrimSpace(input) {
		size, err := strconv.Atoi(string(digit))
		if err != nil {
			fmt.Println("bad character: ", digit)
			continue
		}

		if i%2 == 0 {
			for range size {
				blocks = append(blocks, fileId)
			}
			fileId++
		} else {
			for range size {
				blocks = append(blocks, -1)
			}
		}
	}

	for {
		leftmostFreeSpaceBlock := getLeftmostFreeSpaceBlock(blocks)
		if leftmostFreeSpaceBlock == -1 {
			break
		}
		endOfDiskBlock := getEndOfDiskBlock(blocks)
		if endOfDiskBlock == -1 || endOfDiskBlock <= leftmostFreeSpaceBlock {
			break
		}

		blocks[leftmostFreeSpaceBlock] = blocks[endOfDiskBlock]
		blocks[endOfDiskBlock] = -1
	}

	return checksum(blocks)
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What is the resulting filesystem checksum?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
