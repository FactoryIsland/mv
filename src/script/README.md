# MASM specification

# Commands:
- NOP: No operation
- END: Ends the program
- MOV \<id> \<val>: Moves the \<val> into the variable with index \<id>
- JMP \<id>: Jumps to the instruction at index \<id>
- JZ \<val> \<id>: Performs JMP if the value of the \<val> is 0
- CMP \<val> \<val>: Compares the two \<val> values and stores the result in an accumulator
- JE \<id>: Performs JMP if the comparison result is EQUALS
- JNE \<id>: Performs JMP if the comparison result is NOT_EQUALS
- JG \<id>: Performs JMP if the comparison result is GREATER_THAN
- JGE \<id>: Performs JMP if the comparison result is GREATER_THAN or EQUALS
- JL \<id>: Performs JMP if the comparison result is LESS_THAN
- JLE \<id>: Performs JMP if the comparison result is LESS_THAN or EQUALS
- CALL \<val>: Calls a function (or subroutine)
- RET: Returns from a subroutine (to the last value in the call stack)
- PUSH \<val>: Pushes a value to the function stack (only used for function arguments)
- POP \<id>: Pops a value from the function stack into the variable with index \<id>
- PRINT \<val>: Prints the \<val> to stdout
- SH \<val>: Executes the shell command provided in \<val>

# Variable specification:
- \<id>: just a number
- \<val>:
  - #"": String literal
  - $\<id>: Variable reference (index \<id>)
  - %\<id>: Program argument (index \<id>)
  - \<id>: Integer literal
  - \<float>: Float literal (contains '.')
  - true/false: Boolean

# MV bytecode specification:

Each MASM command has a byte id, which is defined in consts.rs and might change between versions.

Arguments are passed directly after the codec, since every command has a known number of arguments.

Variable identifiers are slightly different:
- #\<u32>\<[u8; u32]>: String literal, the first value being the length, the rest is UTF-8 encoded string data.
- $\<u32>: Variable reference
- %\<u32>: Program argument
- i\<i32>: Integer literal
- f\<f32>: Float literal
- 0u8: false
- 1u8: true

JMP instruction takes in a u32 argument, which is a pointer to what byte of the bytecode to jump to, not what instruction, as this executor has no knowledge of what instruction.

