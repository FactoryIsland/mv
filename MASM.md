# MASM Documentation

MASM is a custom assembly language that compiles to bytecode through the use of the 'mvc' command line tool. The 'mvc' tool also allows you to execute the bytecode.

### Datatypes

- Integer: 64-bit signed integer. Is represented by a simple number without a '.', for example `10`.
- Float: 64-bit floating point. Is represented by a simple number with a '.', for example `10.0`.
- Character: Represented as a 32-bit unsigned integer for arithmetic. Is represented by a character in single quotes or a number engine in 'c', for example `'a'` or `97c`.
- Boolean: True or false. Is represented by `true` or `false`.
- String: A heap-allocated char array. String literals start with a '#', followed by quotes, for example `#"Hello, World!"`.
- Null: A null pointer. Represented by `null`.
- Reference: This is only made by referencing another variable through the `&` operator. Dereferencing a reference can be done through the `*` operator.

### Arithmetic operations

- `add a b` adds the value of `b` to `a`, mutating `a`.
- `sub a b` subtracts the value of `b` from `a`, mutating `a`.
- `mul a b` multiplies the value of `a` by `b`, mutating `a`.
- `div a b` divides the value of `a` by `b`, mutating `a`.
- `mod a b` stores the modulus of `a` by `b` in `a`, mutating `a`.
- `and a b` carries out a bitwise and operation on `a` and `b`, storing the result in `a`.
- `or a b` carries out a bitwise or operation on `a` and `b`, storing the result in `a`.
- `not a` carries out a bitwise not operation on `a`, mutating it.
- `neg a` negates the value of `a`, mutating it.
- `xor a b` carries out a bitwise xor operation on `a` and `b`, storing the result in `a`.
- `shl a b` shifts the value of `a` to the left by the value of `b`, mutating `a`.
- `shr a b` shifts the value of `a` logically to the right by the value of `b`, mutating `a`.
- `sar a b` shifts the value of `a` arithmetically to the right by the value of `b`, mutating `a`.

### Variables

Variables are declared using the `mov` keyword. The syntax is `mov <variable> <value>`. The variable name comes first, followed by its value.

- `mov a 1` assigns the integer `1` to `a`.
- `mov a $b` moves the value of `b`, into `a`, making the value of `b` become `null`.
- `mov a &b` moves a reference to `b` into `a`. Changing `*a` or calling arithmetic operations on `a` will change the value of `b`
- `mov a *b` if `b` is a reference, this will copy the value of the reference into `a`.
- `cpy a $b` copies the value of `b` into `a`, keeping the original value of `b` unchanged.

### Named Variables

Named variables can be enabled using the `.named` keyword. It must be the first line of the file, without trailing whitespace. This is required for linking multiple files. If `.named` is not used, variables are all global and accessed through indexes like a massive array.

**IMPORTANT:** Currently named variable is required! Your code will not compile unless you use named variables.

### Functions

Functions are declared using the `@` symbol followed by the function name. The colon following the function name is optional.

- `@main:` declares a function named `main`.
- Functions are called using the `call` keyword. For example, `call my_function` calls the function `my_function`.
- Functions can return a value using `push_ret`. For example, `push_ret 10` pushes the integer `10` to the return stack. In the caller function, you can use `pop_ret <var_name` to pop the values into a variable.
- Functions can take arguments through the stack. Before calling a function, you can call `push 10` to push the integer `10` as an argument. Use `pop <var_name>` to pop the values into a variable. **NOTE:** Variables are popped in reverse order.
- Use the `ret` instruction to return from a function. This will automatically jump back to the caller. In the main function, this is equivalent to calling `end`.

### Labels

Labels are declared using the `.` symbol followed by the label name. The colon following the label is optional.

- `.label:` declares a label named `label`.
- Labels can be used for jumping and conditional execution, like if statements or loops.

### Jumping

Jumping to labels or instructions is done using the `jmp` keyword.

- `jmp label` jumps to the label named `label`.
- `jmp +2` jumps forward by two instructions.
- `jmp -2` jumps backward by two instructions.
- `jmp 10` jumps to the tenth instruction in the file. **WARNING**: While this is possible, and is the final compiler output, it is highly advised against, as when using multiple modules, this number can be incorrect! Use labels or relative jumps instead.

### Conditionals

Conditional jumps are done using the `j` keyword followed by a condition.

- `jg <to>` jumps if the previous comparison was greater than.
- `jge <to>` jumps if the previous comparison was greater than or equal.
- `jl <to>` jumps if the previous comparison was less than.
- `jle <to>` jumps if the previous comparison was less than or equal.
- `je <to>` jumps if the previous comparison was equal.
- `jne <to>` jumps if the previous comparison was not equal.
- `jn $x <to>` jumps if variable `x` is null.
- `jnn $x <to>` jumps if variable `x` is not null.
- `jz $x <to>` jumps if variable `x` is zero.
- `jnz $x <to>` jumps if variable `x` is not zero.

### External Modules

External files can be included using the `.extern` keyword followed by the file name.

- `.extern my_library.masm` includes the file `my_library.masm`, allowind you to call all the functions in it.

Functions in modules you include must have unique names across all used modules. Global variables and labels however are specific to each module.

### Global Variables

Global variables can be declared using the `.global` keyword followed by the variable name.

- `.global global_variable` declares a global variable named `global_variable`.

Global variables are shared throughout one file.

### Library Functions

Library functions can be imported using the `.use` keyword followed by the function name.

- `.use git_add_all` imports the library function `git_add_all`, which can later be called.

### Shell Commands

Shell commands can be executed using the `sh` keyword followed by the command. Sh can only accept values of type `string`, so this will throw an error if the value is not a string.

- `sh #"echo Hi"` executes the shell command `echo Hi`.

### Output

You can print to the standard output using the `print` keyword followed by what you would like to output. Print can accept all value datatypes, including null and pointers.

- `print #"Hello world!"` prints "Hello world!" to stdout.
