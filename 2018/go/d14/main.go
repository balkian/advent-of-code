package main

import (
	"fmt"
	"strconv"
)

func step(i int, s []int, c []int) (ns []int, nc []int, added int) {
	var t int

	if i != len(c) {
		c[i] = (c[i] + 1 + s[c[i]]) % len(s)
		return s, c, 0
	}
	t = 0
	for _, v := range c {
		t += s[v]
	}
	extra := int2arr(t)
	for j := len(extra) - 1; j >= 0; j-- {
		s = append(s, extra[j])
	}
	return s, c, len(extra)
}

func int2arr(t int) []int {
	if t == 0 {
		return []int{0}
	}
	extra := make([]int, 0)
	for t > 0 {
		extra = append(extra, t%10)
		t = t / 10
	}
	return extra
}

func score(n int) (int, *[]int) {
	s := []int{3, 7}
	c := []int{0, 1}
	l := len(c) + 1
	for i := len(c); len(s) < 10+n; i = (i + 1) % l {
		s, c, _ = step(i, s, c)
		// fmt.Println(s, c, i)
	}

	score := 0

	for i := n; i < n+10; i++ {
		score = score*10 + s[i]
	}
	return score, &s

}

func backwards(n string) int {
	s := []int{3, 7}
	c := []int{0, 1}
	l := len(c) + 1

	target := make([]int, len(n))
	for i, v := range n {
		t, err := strconv.Atoi(string(v))
		if err != nil {
			panic(err)
		}
		target[i] = t
	}

	var added int
	for i := len(c); true; i = (i + 1) % l {
		s, c, added = step(i, s, c)
		if len(s) < len(target) {
			continue
		}

	loop:
		for off := 0; off < added; off++ {

			for j := 0; j < len(target); j++ {
				n1 := s[len(s)-1-j-off]
				n2 := target[len(target)-1-j]
				if n1 != n2 {
					continue loop
				}
			}
			return len(s) - len(target) - off
		}
	}
	return 0
}

func main() {

	s, _ := score(147061)
	fmt.Println(147061, s)

	s = backwards("147061")
	fmt.Println(147061, s)

}
