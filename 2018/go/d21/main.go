package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"strings"
	"time"
	"unicode"
)

type Registers [6]int

type Instruction struct {
	Opcode int
	Args
}

type NamedInstruction struct {
	Name string
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
	return r.ExecName(name, i.Args)
}

func (r Registers) ExecName(name string, a Args) (Registers, error) {
	lower := strings.ToLower(name)
	// Quick hack. I could use reflection to only allow method calls with a given signature
	if lower[:4] == "exec" || lower[:3] == "run" {
		return Registers{}, fmt.Errorf("you cannot call exec/run methods like this")

	}
	name = strings.Title(name)
	method := reflect.ValueOf(r).MethodByName(name)
	res := method.Call([]reflect.Value{reflect.ValueOf(a)})

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

type Program struct {
	reg Registers
	ip  int
	ins []NamedInstruction
}

func (p *Program) Step() bool {
	n := p.reg[p.ip]
	if n >= len(p.ins) {
		return false
	}
	i := p.ins[n]
	res, err := p.reg.ExecName(i.Name, i.Args)
	if err != nil {
		fmt.Printf("error executing %s: %s\n", i, err)
		return false
	}
	res[p.ip]++
	p.reg = res
	return true
}

func ReadProgram() Program {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	s := bufio.NewScanner(f)

	ins := make([]NamedInstruction, 0)
	s.Scan()
	var ip int
	r, err := fmt.Sscanf(s.Text(), "#ip %d", &ip)
	if r != 1 || err != nil {
		panic("could not read ip")
	}
	for s.Scan() {
		i := NamedInstruction{}
		r, err := fmt.Sscanf(s.Text(), "%s %d %d %d", &i.Name, &i.Args.A, &i.Args.B, &i.Args.C)
		if r != 4 || err != nil {
			panic(fmt.Errorf("could not read line: %s: %s", err, s.Text()))
		}
		ins = append(ins, i)
	}
	fmt.Println("Loaded: ", len(ins))
	reg := Registers{}
	return Program{ip: ip, reg: reg, ins: ins}

}

func main() {
	p := ReadProgram()
	// fmt.Println(p.reg, p.ins)
	fmt.Println(p.reg[p.ip], p.ins[p.reg[p.ip]], p.reg)
	for i := 0; i < 100 && p.Step(); i++ {
		fmt.Println(p.reg[p.ip], "\t", p.ins[p.reg[p.ip]], "\t", p.reg)
		time.Sleep(100 * time.Millisecond)
	}
	fmt.Println(p.reg)
	p = ReadProgram()
	r := p.reg
	r[0] = 1
	p.reg = r
	for i := 0; i < 100 && p.Step(); i++ {
		fmt.Println(p.reg[p.ip], "\t", p.ins[p.reg[p.ip]], "\t", p.reg)
		time.Sleep(100 * time.Millisecond)
	}
}
