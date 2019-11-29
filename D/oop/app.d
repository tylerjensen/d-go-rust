module app;

import std.stdio;
import std.datetime;
import per;
import emp;

void main()
{
    writeln("OOP in D");

    Employee a = new Employee(Date.fromISOString("19700508"));
    
    // Composition - Demographics object in Employee
    writeln("Age: ", a.Demo.GetAge());

    // Polymorphism as base class
    poly(a);

    // Polymorphism as interface
    poly(a.Printable);

    // Calls the overridden method
    a.Print(); 

    // Calls the parent class method
    a.Person.Print(); 
    
    // Both call the interface method implemented by Employee
    a.Printable.Print(); 
    a.Person.Printable.Print();

    writeln("Hit Enter to quit.");
    readln();
}

void poly(Person p)
{
    writeln("poly:");
    // The object's overridden method is called
    p.Print();
}

// Overloaded method
void poly(Printable p)
{
    writeln("poly with interface:");
    // The object's implementation is called
    p.Print();
}