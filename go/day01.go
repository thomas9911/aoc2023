package day01

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

var numbers = []string{
	"0", "1", "2", "3", "4",
	"5", "6", "7", "8", "9",
}

var numbersStrings = []string{
	"zero", "one", "two", "three", "four",
	"five", "six", "seven", "eight", "nine",
}

func loopOverLines(filepath string, f func(string) error) error {
	file, err := os.Open(filepath)
	if err != nil {
		return err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		err := f(scanner.Text())
		if err != nil {
			return err
		}
	}

	if err := scanner.Err(); err != nil {
		return err
	}

	return nil
}

func replaceNumbertextToNumber(data string) string {
	for i, item := range numbersStrings {
		data = strings.ReplaceAll(data, item, fmt.Sprintf("%s%d%s", item, i, item))
	}
	return data
}

func getFirstLastNumber(line string) (int, error) {
	var pair []string
	for i := 0; i < len(line); i++ {
		if slices.Contains(numbers, string(line[i])) {
			pair = append(pair, string(line[i]))
			break
		}
	}

	for i := len(line) - 1; i >= 0; i-- {
		if slices.Contains(numbers, string(line[i])) {
			pair = append(pair, string(line[i]))
			break
		}
	}

	stringNumber := strings.Join(pair, "")
	if stringNumber != "" {
		number, err := strconv.Atoi(stringNumber)
		if err != nil {
			return 0, err
		}
		return number, nil
	}
	return 0, nil
}

func Day01a(filepath string) (int, error) {
	score := 0
	err := loopOverLines(filepath, func(line string) error {
		number, err := getFirstLastNumber(line)
		if err != nil {
			return err
		}
		score += number
		return nil
	})

	if err != nil {
		return 0, err
	}

	return score, nil
}

func Day01b(filepath string) (int, error) {
	score := 0
	err := loopOverLines(filepath, func(line string) error {
		line = replaceNumbertextToNumber(line)
		number, err := getFirstLastNumber(line)
		if err != nil {
			return err
		}
		score += number
		return nil
	})

	if err != nil {
		return 0, err
	}

	return score, nil
}
