package main

import "fmt"

func main() {
	fmt.Printf("OOP in Go\n")

	var name string
	fmt.Printf("Hello, what's your name? ")
	fmt.Scanf("%s\n", &name)
	fmt.Printf("Hello, %s\n", name)
}
