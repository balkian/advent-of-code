package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

type status int

type worker struct {
	id        int
	available int
	task      string
}

const (
	pending status = iota
	doing
	done
)

type step struct {
	name         string
	requirements []string
}

func (s *step) String() string {
	return fmt.Sprintf("%s", s.name)
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("Could not read input")
	}
	steps := map[string]*step{}
	s := bufio.NewScanner(f)
	for i := 0; s.Scan(); i++ {
		var name, pre string
		fmt.Sscanf(s.Text(), "Step %s must be finished before step %s can begin.", &pre, &name)
		this := getOrCreate(steps, name)
		// fmt.Println(name, pre)
		if pre == "" {
			continue
		}
		req := getOrCreate(steps, pre)
		this.requirements = append(this.requirements, req.name)
	}
	list := make([]string, 0, len(steps))
	for _, step := range steps {
		list = append(list, step.name)
	}
	sort.Slice(list, func(i, j int) bool { return list[i] < list[j] })
	chain := order(list, steps)
	fmt.Println("Order tasks single worker: ", strings.Join(chain, ""))
	fmt.Println("Multiple workers: ", timeWorkers(list, steps, 5))
}

func order(list []string, steps map[string]*step) []string {
	st := map[string]status{}
	for _, name := range list {
		st[name] = pending
	}

	result := make([]string, 0, len(list))
	left := len(list)

	for left > 0 {
		for _, name := range list {
			if name == "" {
				continue
			}
			t := steps[name]
			if st[name] != pending {
				continue
			}
			doable := true
			for _, r := range t.requirements {
				if st[r] != done {
					doable = false
					break
				}
			}
			if doable {
				st[name] = done
				result = append(result, string(t.name))
				steps[name] = t
				left--
			} else {
			}
		}
	}
	return result

}

func timeWorkers(list []string, steps map[string]*step, n int) int {
	st := map[string]status{}
	for _, name := range list {
		st[name] = pending
	}
	workers := make([]*worker, 0, 5)
	available := make([]*worker, 0, 5)
	for i := 0; i < 5; i++ {
		w := worker{id: i}
		workers = append(workers, &w)
		available = append(available, &w)
	}
	left := len(list)
	now := 0

	for left > 0 {
		for _, name := range list {
			if len(available) < 1 {
				break
			}
			if name == "" {
				continue
			}

			step := steps[name]

			if st[name] != pending {
				continue
			}

			doable := true
			for _, req := range step.requirements {
				if st[req] != done {
					doable = false
					break
				}
			}
			if doable {
				w := available[0]
				st[name] = doing
				w.task = step.name
				w.available = now + 61 + int(step.name[0]-'A')
				// fmt.Printf("Worker %d will be available at %d\n", w.id, w.available)
				available = available[1:]
			}
		}
		now++
		for _, w := range workers {
			if w.available <= now && w.task != "" {
				available = append(available, w)
				st[w.task] = done

				w.task = ""
				left--
			}
		}
	}
	return now
}

func getOrCreate(steps map[string]*step, name string) *step {
	this, ok := steps[name]
	if !ok {
		this = &step{name: name, requirements: make([]string, 0)}
		steps[name] = this
	}
	return this

}
