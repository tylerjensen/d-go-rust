package per

import (
	"fmt"
	"time"
)

type Printable interface {
	Print()
}

type Person struct {
	Name string
}

func (p Person) Print() {
	fmt.Printf("Person printed.\n")
}

type Demographics struct {
	age         int
	DateOfBirth time.Time
}

func (d Demographics) GetAge() int {
	t := time.Now()
	d.age = t.Year() - d.DateOfBirth.Year()
	return d.age
}
