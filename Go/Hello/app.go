package main

import "fmt"

func main() {
	var name string
	fmt.Printf("Hello, what's your name? ")
	fmt.Scanf("%s\n", &name)
	fmt.Printf("Hello, %s\n", name)
}
