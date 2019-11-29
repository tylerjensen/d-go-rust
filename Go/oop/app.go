package main

import (
	"fmt"
	"time"

	"./emp"
	"./per"
)

func main() {
	fmt.Printf("OOP in Go\n")

	const shortForm = "2006-Jan-02"
	t, err := time.Parse(shortForm, "1970-May-08")
	if err != nil {
		//handle err
	}
	d := per.Demographics{DateOfBirth: t}
	a := emp.Employee{} // Cannot use Name (a promoted field) in struct literal
	a.Name = "Tyler"
	a.Demo = d

	// Composition - Demographics object in Employee
	fmt.Printf("Age: %v\n", a.Demo.GetAge())

	// Polymorphism not supported in Go for struct embedding
	poly(a.Person) // Cannot call poly(a)

	// Polymorphism as interface with pointer to Employee object
	var p per.Printable
	p = &a
	poly_p(p)

	// Calls the overridden method
	a.Print()

	// Calls the parent class method
	a.Person.Print()

	// Go does not support direct access to the interface
	// as it is just an implementation pattern match.

	fmt.Printf("Hit Enter to quit.")
	fmt.Scanf("\n")
}

func poly(p per.Person) {
	fmt.Printf("poly:\n")
	// The object's overridden method is called
	p.Print()
}

// Overloading NOT supported in Go
func poly_p(p per.Printable) {
	fmt.Printf("polyp with interface:\n")
	// The object's implementation is called
	p.Print()
}
