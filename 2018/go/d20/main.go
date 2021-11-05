package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strings"
)

var (
	open     = byte('^')
	close    = byte('$')
	openPar  = byte('(')
	closePar = byte(')')
	div      = byte('|')
)

type xy struct {
	x, y int
}

type node interface {
	dist() int
	empty() bool
	count(int) int
	walk(xy, int, map[xy]int) (xy, int)
}

func dist(s string) int {
	n, r := parse(s)
	if r < len(s)-1 {
		panic(fmt.Errorf("not fully read %s %s %d/%d", s, n, r, len(s)))

	}
	// fmt.Printf("%#v\n", n)
	return n.dist()
}

func parse(s string) (result node, read int) {
	var (
		opened = false
		c      node
	)

	for read = 0; read < len(s); read++ {
		if s[read] == open {
			opened = true
			read++
			break
		}
	}
	c, delta := newChain(s[read:])
	read = read + delta
	if !opened {
		panic(fmt.Errorf("unopened chain: %s", s))

	}
	if s[read] != close {
		panic(fmt.Errorf("unclosed chain: %c @ %d", s[read], read))
	}

	return c, read
}

type leaf struct {
	value string
}

func (l leaf) empty() bool {
	return len(l.value) == 0
}

func (l leaf) dist() int {
	// fmt.Println("checking dist for leaf", l.value)
	// time.Sleep(time.Millisecond * 200)
	return len(l.value)
}

func (l leaf) count(d int) int {
	c := 0
	for i := 0; i < len(l.value); i++ {
		d--
		if d <= 0 {
			c++
		}
	}
	return c
}

func (l leaf) walk(pos xy, d int, dm map[xy]int) (xy, int) {
	for _, r := range l.value {
		d++ //opening a door
		switch r {
		case 'W':
			pos = xy{pos.x - 1, pos.y}
		case 'E':
			pos = xy{pos.x + 1, pos.y}
		case 'N':
			pos = xy{pos.x, pos.y - 1}
		case 'S':
			pos = xy{pos.x, pos.y + 1}
		}
		if cd, ok := dm[pos]; !ok || cd > d {
			dm[pos] = d
		}
	}
	return pos, d

}

var leafc = 0

func newLeaf(s string) (result leaf, read int) {
	leafc++
	var (
		r   byte
		cur = strings.Builder{}
	)

loop:
	for read = 0; read < len(s); read++ {
		r = s[read]
		// fmt.Printf("Leaf %2d %c\n", read, r)
		switch r {
		case close:
			fallthrough
		case div:
			fallthrough
		case open:
			fallthrough
		case openPar:
			fallthrough
		case closePar:
			break loop
		case 'N':
			fallthrough
		case 'S':
			fallthrough
		case 'W':
			fallthrough
		case 'E':
			cur.WriteByte(r)
		default:
			panic(fmt.Errorf("unknown character for leaf %c", r))
		}
	}
	// fmt.Println("end leaf")
	return leaf{cur.String()}, read
}

type chain struct {
	children []node
}

func (c chain) empty() bool {
	for _, ch := range c.children {
		if !ch.empty() {
			return false
		}
	}
	return true
}

func (c chain) dist() int {
	sum := 0
	for _, n := range c.children {
		sum += n.dist()
	}
	return sum
}

func (c chain) walk(pos xy, d int, dm map[xy]int) (xy, int) {
	for i := 0; i < len(c.children); i++ {
		pos, d = c.children[i].walk(pos, d, dm)
	}
	return pos, d
}

func (c chain) count(d int) int {
	sum := 0
	for i := 0; i < len(c.children); i++ {
		sum += c.children[i].count(d)
		d -= c.dist()
	}
	return sum
}

var chainc = 0

func newChain(s string) (result chain, read int) {
	chainc++
	var (
		r        byte
		delta    int
		children = make([]node, 0)
		c        node
	)

loop:
	for read = 0; read < len(s); read++ {
		// time.Sleep(200 * time.Millisecond)
		r = s[read]
		// fmt.Printf("Chain %2d %c\n", read, r)
		switch r {
		case close:
			fallthrough
		case div:
			break loop
		case openPar:
			// fmt.Println("Opening pars")
			c, delta = newOpts(s[read:])
			read += delta - 1
			children = append(children, c)
		default:
			c, delta = newLeaf(s[read:])
			if delta <= 0 {
				// fmt.Println("Chain got somewhere it does not understand")
				break loop
			}
			read += delta - 1
			// fmt.Println("adding child", c)
			children = append(children, c)
		}
	}
	// fmt.Println("end chain", len(children))
	return chain{children: children}, read
}

type opts struct {
	children []node
}

func (n opts) empty() bool {
	if len(n.children) == 0 {
		return true
	}

	l := len(n.children) - 1
	if n.children[l].empty() {
		return true
	}
	return false
}

func (n opts) dist() int {
	var (
		t   int
		max = 0
	)

	if n.empty() {
		return 0
	}
	for _, c := range n.children {
		t = c.dist()
		if t > max {
			max = t
		}
	}
	return max
}

func (n opts) walk(pos xy, d int, dm map[xy]int) (xy, int) {
	var (
		minpos xy
		min    = math.MaxInt64
		dc     int
		posc   xy
	)
	for i := 0; i < len(n.children); i++ {
		posc, dc = n.children[i].walk(pos, d, dm)
		if dc < min {
			min = dc
			minpos = posc
		}
	}
	return minpos, dc
}

func (n opts) count(d int) int {

	c := 0
	shortcut := false

	if len(n.children) == 0 {
		return 0
	}

	l := len(n.children) - 1
	if n.children[l].empty() {
		shortcut = true
	}

	for i := 0; i < len(n.children); i++ {
		if shortcut {
			c += n.children[i].count(d) - 1
		} else {
			c += n.children[i].count(d) - 1
		}
	}
	return c
}

var optc = 0

func newOpts(s string) (result opts, read int) {
	optc++

	var (
		r      byte
		o      = make([]node, 0)
		delta  int
		opened = false
		closed = false
		c      chain
	)

loop:
	for ; read < len(s); read++ {
		r = s[read]
		// fmt.Printf("Opts: %d %c\n", read, r)
		switch r {
		case close:
			break loop
		case openPar:
			opened = true
			fallthrough
		case div:
			if !opened {
				panic("options without opening first")
			}
			c, delta = newChain(s[read+1:])
			read += delta
			o = append(o, c)
			// fmt.Printf("Adding chain %#v\n", c)
		case closePar:
			closed = true
			read++
			break loop
		default:
			panic(fmt.Errorf("unknown character for opts %c", r))
		}
	}
	if !closed {
		panic(fmt.Errorf("Opts not closed %s", o))
	}
	// fmt.Println("end opts")
	return opts{children: o}, read
}

func main() {
	b, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("could not read file")
	}
	s := strings.TrimSpace(string(b))
	r, _ := parse(s)
	fmt.Println("Solution using dist():", r.dist())

	m := map[xy]int{}
	r.walk(xy{}, 0, m)
	var (
		max   int
		count int
	)
	for _, d := range m {
		if d > max {
			max = d
		}
		if d >= 1000 {
			count++
		}

	}
	fmt.Println("Solutions with a walker: ", max, count)
}
