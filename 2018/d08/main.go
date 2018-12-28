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
	license := strings.Split(string(b), " ")
	chunks := make([]int, len(license))
	for k, v := range license {
		i, err := strconv.Atoi(v)
		if err != nil {
			panic("invalid int")
		}
		chunks[k] = i
	}
	fmt.Println(license)

	_, n := chunksToNode(chunks)
	fmt.Printf("%s", n)

}

func chunksToNode(chunks []int) (read int, node Node) {
	nc := chunks[0]
	nm := chunks[1]
	node = *NewNode(nc, nm)
	for i := len(chunks) - nm; i < len(chunks); i++ {
		fmt.Println(len(chunks), i, nm)
		node.metadata = append(node.metadata, chunks[i])
	}
	chunks = chunks[2 : len(chunks)-nm]

	var idx = 0
	for idx < len(chunks) {
		delta, child := chunksToNode(chunks)
		node.children = append(node.children, child)
		idx += delta
		chunks = chunks[idx:len(chunks)]
	}

	return idx + 2 + nm, node

}
