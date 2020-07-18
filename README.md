# jamcc
A Compiler written for a small subset of C written in Rust.

## How do you run it?
To call the compiler run:
```
target/release/jamcc <.c file>
```
It will generate a test.s file in your current directory. Then you can call 
```
gcc test.s
```
to generate an a.out binary that you can run.

## Why?
I was inspired by my team's work on a [custom-kinda-brainfuck compiler for a hackathon.](https://github.com/brandonspark/Runtime) 

## Why Rust?
I wanted to write it in SML but I was dissuaded by some friends. Maybe in the future.

## What can it do?
It can generate code for a main function, returning an integer. When I say small subset, I mean small.
```
int main()
    {
        return 200;
    }
```

## What's next?
I plan to add some unary operators. Soon tm.
