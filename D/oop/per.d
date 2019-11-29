module per;

import std.stdio;
import std.datetime;

public interface Printable 
{
    void Print();
} 

public abstract class Person : Printable
{
    public void Print() 
    { 
        writeln("Person printed.");
    }
    public string Name;
}

public class Demographics
{
    private int age;
    public Date DateOfBirth;

    this(Date dob)
    {
        DateOfBirth = dob;
    }

    public int GetAge()
    {
        SysTime today = Clock.currTime;
        
        // Example of open recursion - 'this'
        this.age = today.year - DateOfBirth.year;
        
        return age;
    }
}
