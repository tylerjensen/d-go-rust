module emp;
import per;

import std.stdio;
import std.datetime;

public final class Employee : Person
{
    public Demographics Demo;

    this(Date dob)
    {
        Demo = new Demographics(dob);
    }

    public override void Print()
    {
        writeln("Employee printed.");
        writeln("Age: ", Demo.GetAge());
        // These next 2 lines throw compiler error as data is private
        // Demo.age = 44; 
        // writeln("Age: ", Demo.age);
    }
}

// Throws as cannot inherit from final Employee class
// public class EmployeeExt : Employee { }
