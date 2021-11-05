package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strings"
)

const gens1 = 20
const gens2 = 50000000000
const rulelength = 5
const hasPlant = '#'
const noPlant = '.'

type State map[int]bool

func NewState(f string) (s State) {
	s = State{}
	for i := 0; i < len(f); i++ {
		if f[i] == hasPlant {
			s[i] = true
		}
	}
	return s
}

func (s State) String() string {
	w := strings.Builder{}
	shape, min := s.Shape()
	w.WriteString(fmt.Sprintf("%d: ", min))
	for _, val := range shape {
		if val {
			w.WriteRune(hasPlant)
			continue
		}
		w.WriteRune(noPlant)
	}
	return w.String()
}

func (s State) Shape() (mask, int) {
	min, max := math.MaxInt64, 0
	for ix, val := range s {
		if !val {
			continue
		}
		if ix < min {
			min = ix
		}
		if ix > max {
			max = ix
		}
	}
	shape := make(mask, 0)
	for i := min; i <= max; i++ {
		shape = append(shape, s[i])
	}
	return shape, min

}

func (s State) Count() (sum int) {
	for ix, val := range s {
		if val {
			sum += ix
		}
	}
	return sum
}

func (s State) Compare(o State) (bool, int) {
	if len(s) != len(o) {
		return false, 0
	}
	s0, m0 := s.Shape()
	s1, m1 := o.Shape()
	for ix, val := range s0 {
		if val != s1[ix] {
			return false, 0
		}
	}
	return true, m0 - m1
}

func (s State) Shift(delta int) State {
	n := State{}
	for ix, val := range s {
		n[ix+delta] = val
	}
	return n
}

type mask []bool

func (m mask) String() string {
	s := strings.Builder{}
	for _, val := range m {
		if val {
			s.WriteRune(hasPlant)
			continue
		}
		s.WriteRune(noPlant)
	}
	return s.String()
}

type Rule struct {
	mask   mask
	offset int
	val    bool
}

func NewRule(s string, r string) Rule {
	m := make(mask, len(s))
	for v, val := range s {
		if val == hasPlant {
			m[v] = true
		}
	}
	return Rule{mask: m,
		offset: 0,
		val:    r[0] == hasPlant}
}

func (r Rule) Trim() Rule {
	if r.offset >= len(r.mask)-1 || r.mask[r.offset] {
		return r
	}
	return Rule{mask: r.mask, offset: r.offset + 1, val: r.val}.Trim()
}

func (r Rule) String() string {
	return fmt.Sprintf("%s -> %t @ %d", r.mask, r.val, r.offset)
}

type Rules []Rule

func (s State) Check(rule Rule, pos int) (bool, int) {
	start := pos - rule.offset
	for ix, m := range rule.mask {
		v := s[start+ix]
		if v != m {
			return false, pos
		}
	}
	return true, start + 2

}

func (s State) Step(rules Rules) State {
	result := State{}

	for pos := range s {
		for _, rule := range rules {
			ok, dst := s.Check(rule, pos)
			if !ok {
				continue
			}
			result[dst] = true
		}
	}
	return State(result)
}

func main() {
	var initial State
	rules := Rules{}

	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	s := bufio.NewScanner(f)
	var in string
	s.Scan()
	_, err = fmt.Sscanf(s.Text(), "initial state: %s", &in)
	if err != nil {
		panic("could not read initial state")
	}
	initial = NewState(in)

	fmt.Println(initial)
	s.Scan() // empty line
	var condition string
	var result string

	for s.Scan() {
		_, err := fmt.Sscanf(s.Text(), "%s => %s", &condition, &result)
		if err != nil {
			panic(fmt.Sprintf("could not read rule %s", err))
		}
		rule := NewRule(condition, result).Trim()
		if rule.val {
			rules = append(rules, rule)
		}

	}
	sort.Slice(rules, func(i, j int) bool { return rules[i].offset > rules[j].offset })

	first := initial
	for i := 0; i < gens1; i++ {
		first = first.Step(rules)
	}
	fmt.Println("First answer: ", first.Count())

	second := initial
	total := 0
	delta := 0
	for i := 0; i < gens2; i++ {
		n := second.Step(rules)
		eq, dist := n.Compare(second)
		if eq {
			delta = n.Count() - second.Count()
			fmt.Printf("Equal shape. Delta: %10d . Distance: %2d\n", delta, dist)
			total += delta * (gens2 - i)
			break
		}
		second = n
		total = second.Count()

		if i%10 == 0 {
			fmt.Println(second)
			fmt.Printf("Total@%d: %d\n", i, total)
		}
	}
	fmt.Println("Total: ", total)
}
