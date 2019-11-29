package emp

import (
	"fmt"

	"../per"
)

type Employee struct {
	per.Person
	Demo per.Demographics
}

func (e Employee) Print() {
	fmt.Printf("Employee printed.\n")
	fmt.Printf("Age: %v\n", e.Demo.GetAge())
}
