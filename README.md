# bool-table
bool-table is a bool based esoteric programming language, sadly it's not turing complete but it can compute any sized output with any sized input, so it can do things like:
- sum
- substract
- multiply
- divide
- square root

# tutorial
basic rules:
- last function is main
- you start with nand and true
- functions are defined the first time they are mentioned

tutorial:

here an example about a basic program in bool-table
```
main T T nand
```
- `main` is mentioned for the first time, so `main` is defined
- 2 `T`'s are added to memory
- `nand` statement is called, and `nand(T, T)` = `false`
- `false` is returned

`+` moves(not add) last memory value into stack, `-` pops last stack value and **numbers** adds last N value in memory
```
not + 0 0 nand -
main not
```
- `not` is mentioned for the first time, so `not` is defined
- `main` is mentioned for the first time, so `main` is defined
- `not` is called with `main`'s arguments
- first argument is added to stack with `+`
- last item in stack(first argument) is added to memory
- last item in stack(first argument) is added to memory(again)
- a nand with the same arguments reverses them (it works with nand(T, 0) too)
- last value in stack(first argument) is popped
- end of `not` function
- end of `main` function
