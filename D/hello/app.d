import std.stdio;

void main()
{
    string name;
    write("Hello, what's your name? ");
    readf("%s\n", &name);
    writeln("Hello, ", name);
}
