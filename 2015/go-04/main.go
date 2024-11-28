package main

import (
	"crypto/md5"
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func PartOne(input string) int {
	secretKey := strings.TrimSpace(input)

	number := 0
	for {
		hashInput := fmt.Sprintf("%s%d", secretKey, number)
		hash := md5.Sum([]byte(hashInput))

		if strings.HasPrefix(fmt.Sprintf("%x", hash), strings.Repeat("0", 5)) {
			break
		}

		number += 1
	}

	return number
}

func PartTwo(input string) int {
	secretKey := strings.TrimSpace(input)

	number := 0
	for {
		hashInput := fmt.Sprintf("%s%d", secretKey, number)
		hash := md5.Sum([]byte(hashInput))

		if strings.HasPrefix(fmt.Sprintf("%x", hash), strings.Repeat("0", 6)) {
			break
		}

		number += 1
	}

	return number
}

func main() {
	fmt.Println("…find MD5 hashes which, in hexadecimal, start with at least five zeroes.", PartOne(input))

	fmt.Println("Now find one that starts with six zeroes.", PartTwo(input))
}
