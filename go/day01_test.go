package day01

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestDay01a(t *testing.T) {
	score, err := Day01a("../data/day01.txt")

	if err != nil {
		t.Fatal(err)
	}

	expected := 55130
	assert.Equal(t, expected, score)
}

func TestDay01b(t *testing.T) {
	score, err := Day01b("../data/day01.txt")

	if err != nil {
		t.Fatal(err)
	}

	expected := 54985
	assert.Equal(t, expected, score)
}
