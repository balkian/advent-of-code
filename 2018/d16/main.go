package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"strings"
	"unicode"
)

type Registers [4]int

type Instruction struct {
	Opcode int
	Args
}

type Args struct {
	A, B, C int
}

func (r Registers) check(addrs ...int) bool {
	for _, a := range addrs {
		if a < 0 || len(r) <= a {
			return false
		}
	}
	return true
}

func (r Registers) Addr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = r[a.A] + r[a.B]
	return out, nil
}

func (r Registers) Addi(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = r[a.A] + a.B
	return out, nil
}

func (r Registers) Mulr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = r[a.A] * r[a.B]
	return out, nil
}

func (r Registers) Muli(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = r[a.A] * a.B
	return out, nil
}

func (r Registers) Banr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = int(byte(r[a.A]) & byte(r[a.B]))
	return out, nil
}

func (r Registers) Bani(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = int(byte(r[a.A]) & byte(a.B))
	return out, nil
}
func (r Registers) Borr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = int(byte(r[a.A]) | byte(r[a.B]))
	return out, nil
}

func (r Registers) Bori(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = int(byte(r[a.A]) | byte(a.B))
	return out, nil
}

func (r Registers) Setr(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = r[a.A]
	return out, nil

}
func (r Registers) Seti(a Args) (Registers, error) {
	if !r.check(a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	out[a.C] = a.A
	return out, nil
}

func (r Registers) Gtir(a Args) (Registers, error) {
	if !r.check(a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if a.A > r[a.B] {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}
func (r Registers) Gtri(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if r[a.A] > a.B {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}
func (r Registers) Gtrr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if r[a.A] > r[a.B] {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}

func (r Registers) Eqir(a Args) (Registers, error) {
	if !r.check(a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if a.A == r[a.B] {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}
func (r Registers) Eqri(a Args) (Registers, error) {
	if !r.check(a.A, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if r[a.A] == a.B {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}
func (r Registers) Eqrr(a Args) (Registers, error) {
	if !r.check(a.A, a.B, a.C) {
		return Registers{}, fmt.Errorf("overflow for %v - %v", r, a)
	}
	out := r
	if r[a.A] == r[a.B] {
		out[a.C] = 1
	} else {
		out[a.C] = 0
	}
	return out, nil
}

func (r Registers) Exec(i Instruction, m map[int]string) (Registers, error) {
	name, ok := m[i.Opcode]
	if !ok {
		panic(fmt.Errorf("Unknown opcode %s", i.Opcode))
	}
	return r.ExecName(name, i)
}

func (r Registers) ExecName(name string, i Instruction) (Registers, error) {
	if strings.ToLower(name)[:len("exec")] == "exec" {
		return Registers{}, fmt.Errorf("you cannot call exec methods like this")

	}
	method := reflect.ValueOf(r).MethodByName(name)
	res := method.Call([]reflect.Value{reflect.ValueOf(i.Args)})

	reg := (res[0].Interface()).(Registers)
	err, ok := (res[1].Interface()).(error)
	if ok {
		return reg, err
	}

	return reg, nil
}

func methods(r interface{}) []string {
	names := make([]string, 0)
	fooType := reflect.TypeOf(r)
	for i := 0; i < fooType.NumMethod(); i++ {
		method := fooType.Method(i)
		if unicode.IsUpper(rune(method.Name[0])) {
			names = append(names, method.Name)
		}
	}
	return names
}

type tcase struct {
	before Registers
	inst   Instruction
	after  Registers
}

func SolveA() map[string]map[int]bool {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	s := bufio.NewScanner(f)

	cases := make([]tcase, 0)
	var inst = Instruction{}
	var bef = Registers{}
	var aft = Registers{}
	for s.Scan() {
		r, err := fmt.Sscanf(s.Text(), "Before: [%d, %d, %d, %d]", &bef[0], &bef[1], &bef[2], &bef[3])
		if r != 4 || err != nil {
			continue
		}
		s.Scan()
		r, err = fmt.Sscanf(s.Text(), "%d %d %d %d", &inst.Opcode, &inst.A, &inst.B, &inst.C)
		if r != 4 || err != nil {
			panic(fmt.Errorf("Error '%s' reading %s\n", err, s.Text()))
		}
		s.Scan()
		r, err = fmt.Sscanf(s.Text(), "After: [%d, %d, %d, %d]", &aft[0], &aft[1], &aft[2], &aft[3])
		if r != 4 || err != nil {
			panic(fmt.Errorf("Error '%s' reading %s\n", err, s.Text()))
		}
		c := tcase{before: bef, after: aft, inst: inst}
		cases = append(cases, c)
	}
	n3 := 0
	matches := map[tcase][]string{}

	canbe := map[string]map[int]bool{}
	names := methods(Registers{})
	for _, name := range names {
		canbe[name] = map[int]bool{}
	}

	for _, c := range cases {
		bef = c.before
		aft = c.after
		inst = c.inst

		for _, n := range names {
			_, ok := canbe[n][inst.Opcode]
			if !ok {
				canbe[n][inst.Opcode] = true
			}
			res, err := bef.ExecName(n, inst)
			if err != nil || res != aft {
				canbe[n][inst.Opcode] = false
				continue
			}
			_, ok = matches[c]
			if !ok {
				matches[c] = make([]string, 0)
			}
			matches[c] = append(matches[c], n)
		}
		if len(matches[c]) >= 3 {
			n3++
		}
	}
	fmt.Println(len(cases), n3)
	return canbe

}

func Resolve(canbe map[string]map[int]bool) map[int]string {
	known := map[int]string{}
	updated := true
	for updated {
		updated = false
		for name, m := range canbe {
			count := 0
			for oc, val := range m {
				if val {
					count++
				} else {
					delete(m, oc)
				}
			}
			if count == 1 {
				var koc int
				for oc := range m {
					koc = oc
				}
				known[koc] = name
				delete(canbe, name)
				for tname, tm := range canbe {
					for toc := range tm {
						if toc == koc {
							delete(canbe[tname], koc)
						}
					}

				}
				updated = true
			}
		}
	}
	if len(known) < 16 {
		panic("No luck!")
	}
	return known
}

func ReadCode() []Instruction {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	s := bufio.NewScanner(f)

	insts := make([]Instruction, 0)
	inst := Instruction{}
	for i := 0; i < 3162; i++ {
		s.Scan()
	}
	for s.Scan() {
		r, err := fmt.Sscanf(s.Text(), "%d %d %d %d", &inst.Opcode, &inst.A, &inst.B, &inst.C)
		if r != 4 || err != nil {
			panic(fmt.Errorf("Error '%s' reading %s\n", err, s.Text()))
		}
		insts = append(insts, inst)
	}
	return insts
}

func main() {
	canbe := SolveA()
	known := Resolve(canbe)
	fmt.Println(known)
	insts := ReadCode()
	reg := Registers{}
	var err error
	for i := 0; i < len(insts); i++ {
		ins := insts[i]
		reg, err = reg.Exec(ins, known)
		if err != nil {
			panic(fmt.Errorf("error with code: %d: %s ", i, err))
		}
	}
	fmt.Println(reg)
}
