package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	combination "github.com/ernestosuarez/itertools"
	iter "github.com/yanatan16/itertools"
)

func main() {

	numbers := []int{
		1721,
		979,
		366,
		299,
		675,
		1456,
	}

	var s []int

	// Open the file.
	f, _ := os.Open("input.txt")
	// Create a new Scanner for the file.
	scanner := bufio.NewScanner(f)
	// Loop over all lines in the file and print them.
	for scanner.Scan() {
		line := scanner.Text()
		n, _ := strconv.Atoi(line)
		s = append(s, n)
	}
	_, _, _, product3_1 := findproduct3(numbers)
	_, _, _, product3_2 := findproduct3(s)

	fmt.Println("product3 test:", product3_1)
	fmt.Println("product3 real:", product3_2)

	productN3_1 := findproductN(numbers, 3)
	productN3_2 := findproductN(s, 3)

	fmt.Println("product3 test2:", productN3_1)
	fmt.Println("product3 real2:", productN3_2)

}

func findproductN(nums []int, n int) int {

	summer := func(a, b interface{}) interface{} {
		return a.(int) + b.(int)
	}
	var sum int
	for v := range combination.CombinationsInt(nums, n) {
		// sum = Reduce(Filter(mod3er, Map(squarer, Count(1))), summer, 0)
		sum = iter.Reduce(iter.New(v), summer, 0).(int)
		fmt.Println(sum)
	}
	return sum
}

func findproduct2(nums []int) (int, int, int) {
	for i, numi := range nums {
		for j, numj := range nums {
			var sum int
			if j != i {
				sum = numj + numi
				if sum == 2020 {
					fmt.Println(numi, "+", numj, "=", numj+numi)
					return numi, numj, numi * numj
				}
			}
		}
	}
	return 0, 0, 0
}

func findproduct3(nums []int) (int, int, int, int) {
	var sum int

	for i, numi := range nums {
		for j, numj := range nums {
			if i == j {
				continue
			}
			for k, numk := range nums {
				if k == j || k == i {
					continue
				}
				sum = numk + numj + numi
				if sum == 2020 {
					fmt.Println(numi, "+", numj, "+", numk, "=", numj+numi+numk)
					return numi, numj, numk, numi * numj * numk
				}
			}
		}
	}
	return 0, 0, 0, 0
}
