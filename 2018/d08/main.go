package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	children []Node
	metadata []int
}

func (n Node) Sum() int {
	sum := 0
 	for _,c := range n.children {
		sum += c.Sum()
	}
	for _, m := range n.metadata {
		sum += m
	}
	return sum
}

func (n Node) Sum2() int {
	if len(n.children) < 1 {
		return n.Sum()
	}

	sum := 0
 	
	for _, m := range n.metadata {
		idx := m-1
		if idx >= len(n.children) {
			continue
		}
		sum += n.children[idx].Sum2()
	}
	return sum
}

func (n Node) String() string {
	s := strings.Builder{}
	prefix := "  "
	s.WriteString(prefix)
	s.WriteString(fmt.Sprintf("[%v]\n", n.metadata))
	for _, child := range n.children {
		s.WriteString(prefix)
		for _, line := range strings.Split(child.String(), "\n") {
			s.WriteString(prefix)
			s.WriteString(line)
			s.WriteString("\n")
		}
	}
	return s.String()
}

func NewNode(nc, nm int) *Node {
	return &Node{children: make([]Node, nc),
		metadata: make([]int, nm)}
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
	license := strings.Split(strings.TrimSpace(string(b)), " ")
	chunks := make([]int, len(license))
	for k, v := range license {
		i, err := strconv.Atoi(v)
		if err != nil {
			panic(fmt.Errorf("invalid int %s @ %s: %s", v, k, err))
		}
		chunks[k] = i
	}
	fmt.Println(license)

	_, n := chunksToNode(chunks)
	fmt.Printf("%s", n)
	fmt.Println(n.Sum())
	fmt.Println(n.Sum2())

}

func chunksToNode(chunks []int) (read int, node Node) {
	nc := chunks[0]
	nm := chunks[1]
	node = *NewNode(nc, nm)
	fmt.Println("Num children: ", nc)
	read = 2
	for i:=0; i<nc;i++ {
		delta, child := chunksToNode(chunks[read:])
		node.children[i] = child
		read += delta
	}
	for i := 0; i < nm; i++ {
		node.metadata[i] = chunks[read]
		read++
	}
	return read, node

}
