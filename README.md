# jamcc
A compiler written for a small subset of C written in Rust.

## How do you run it?
To call the compiler run:
```
target/release/jamcc <file-name>.c
```
To enable debug printing you can add the `--features debug` flag.
It will generate a <file-name>.s file in your current directory. Then you can call: 
```
gcc <file-name>.s
```
to generate an a.out binary that you can run.

## Why?
I was inspired by my team's work on a [custom-kinda-brainfuck compiler for a hackathon.](https://github.com/brandonspark/Runtime) 

## Why Rust?
I wanted to write it in SML but I was dissuaded by some friends. Maybe in the future.

My friend is working on a [SML compiler for C](https://github.com/brandonspark/smlcc), though. Check him out!
## What can it do?
It can generate code for a main function, returning an integer. When I say small subset, I mean small.
```
int main()
    {
        return 200;
    }
```

7/23/2020: It now supports certain binary operators such as: -, ~, and !.

## What's next?
Next up is adding binary operators like +, -, *, /. Stay tuned.
