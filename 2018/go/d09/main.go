package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

type Marble struct {
	left *Marble
	right *Marble
	value int
}

func (m Marble) String() string {
	res := strings.Builder{}
	next := &m
	res.WriteString(fmt.Sprint(next.value))
	next = next.right
	for next != nil &&  *next != m {
	//	fmt.Printf("%p %v", next.right, &m)
		res.WriteString(" ")
		res.WriteString(fmt.Sprint(next.value))
		next = next.right
	}
	return res.String()
}

func (m *Marble) AddLeft(n *Marble) {
	m.left, n.left = n, m.left
	n.right = m
	n.left.right = n
}

func (m *Marble) AddRight(n *Marble) {
	m.right.AddLeft(n)
}

func (m *Marble) Remove() {
	m.right.left = m.left
	m.left.right = m.right
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("Could not read input")
	}
	b, err := ioutil.ReadAll(f)
	if err != nil {
		panic(err)
	}
	var np, max int
	fmt.Sscanf(string(b), "%d players; last marble is worth %d points", &np, &max)
	fmt.Println(np, max)

	scores := make([]int, np)

	current := &Marble{value: 0}
	current.left = current
	current.right = current
	for i:=1; i<max; i++ {
		if i % 23 != 0 {
			new := &Marble{value: i}
			current.right.AddRight(new)
			current = new
		} else {
			player := (i-1) % np
			scores[player] += i
			for i:=0; i<7; i++ {
				current = current.left
			}
			scores[player] += current.value
			current.Remove()
			current = current.right
		}
	}
	winner := 0
	for _, val := range scores {
		if val > winner {
			winner = val
		}
	}
	fmt.Println(winner)

}
