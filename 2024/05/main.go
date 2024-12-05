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

type pageOrderingRule struct {
	before, after string
}

type pagesToProduce struct {
	pageOrder map[string]int
	pages     []string
}

func (u pagesToProduce) isSorted(orderingRules []pageOrderingRule) bool {
	for _, order := range orderingRules {
		left, ok := u.pageOrder[order.before]
		if !ok {
			continue
		}
		right, ok := u.pageOrder[order.after]
		if !ok {
			continue
		}

		if left > right {
			return false
		}
	}

	return true
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))
	orderingRules := []pageOrderingRule{}

	middleSum := 0
	for scanner.Scan() {
		line := scanner.Text()

		if strings.Contains(line, "|") {
			rule := strings.Split(line, "|")
			orderingRules = append(orderingRules, pageOrderingRule{rule[0], rule[1]})
		}

		if strings.Contains(line, ",") {
			update := pagesToProduce{pageOrder: map[string]int{}, pages: []string{}}
			pages := strings.Split(line, ",")

			for number, page := range pages {
				update.pageOrder[page] = number
				update.pages = append(update.pages, page)
			}

			if update.isSorted(orderingRules) {
				middle, err := strconv.Atoi(update.pages[len(pages)/2])
				if err != nil {
					fmt.Println("invalid line:", line, err)
					continue
				}

				middleSum += middle
			}
		}
	}

	return middleSum
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What do you get if you add up the middle page number from those correctly-ordered updates?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
