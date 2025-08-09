# RvmASM Assembler
This is the assembler for ```.rvmasm``` files. It will parse any file of that type and convert it into binary file for [Rusty-VM](https://github.com/LordAfterEight/rusty-vm/blob/master/README.md) to read. to use it, first run the following command:
```shell
cargo install rvmasm
```
Now you can use the ```rvmasm``` command to build a binary from any ```.rvmasm``` input file:
```shell
rvmasm code.rvmasm output
```

# Documentation
RvmASM is an Assembly-ish language for my 16-bit virtual machine Rusty-VM. I made this assembly language and its parser to allow me and maybe even others to easily create programs for the virtual machine without needing to write raw binary values into a file. It is currently under development, just like the virtual machine itself, so both are far from being finished. Under this paragraph you will find a documentation of the entire language. This documentation will constantly change as more features and content are added to the language.

# Table of Contents

### 1. [Keywords](#Keywords)
   - [routine:](#routine)
   - [end](#end)
   - [lit](#lit)
   - [hex](#hex)
   - [num](#num)
   - [str](#str)
   - [col](#col)

### 2. [Routines](#Routines)
   
### 3. [Instructions](#Instructions)


| Jump | Register | Arithmetics | Miscellaneous |
|-------------|-------------|-------------|-------------|
|[jump](#jump)|[load](#load)|[comp](#comp)|[noop](#noop)|
|[jusr](#jusr)|[stor](#stor)|[radd](#radd)|[setv](#setv)|
|[juie](#juie)|             |[rsub](#rsub)|
|[juin](#juin)|             |[rmul](#rmul)|
|[rtor](#rtor)|             |[rdiv](#rdiv)|



## Keywords <a name="Keywords"></a>
These are mostly used to determine how the following value will be interpreted.
There are seven keywords: ```routine:```, ```end```, ```lit```, ```hex```, ```num```, ```str``` and ```col```.

### ```routine:``` <a name="routine"></a>
<details open>
  <Summary> Explanation </Summary>
  
```routine:``` starts the definition of a routine. Example:
```ruby
routine: routine1       # Creates a routine with the name routine1
...
```
</details>

### ```end``` <a name="end"></a>
<details open>
  <Summary> Explanation </Summary>
  
```end``` is used to mark the end of the routine. Example:
```ruby
...
end      # All that's needed to end the routine definition
```
</details>

### ```lit``` <a name="lit"></a>
<details open>
  <Summary> Explanation </Summary>
  
```lit``` will return the value as is (thus "lit" for "literal"), which is why the value must not be longer than four characters and it must not contain any special symbols; only ```0-9```and ```A-F``` are allowed. Especially useful when   you need to specify addresses. Examples:
```ruby
lit 0x0FA3
lit 0FA3
lit FA3
# These are all the same
```
</details>

### ```hex``` <a name="hex"></a>
<details open>
  <Summary> Explanation </Summary>
  
```hex``` Interprets the following value as **character**, converting it to its numerical ASCII representation. Examples:
```ruby
load A hex U        # "U" will be converted to 0x0055 and loaded into Register A
load A lit 0x0055   # Same value
```
</details>

### ```num``` <a name="num"></a>
<details open>
  <Summary> Explanation </Summary>
  
```num``` enables you to use any decimal number from 0 to 65535. Examples:
```ruby
load A num 7        # Number 7 will be loaded into the A register. Would be the same as "lit 0x0007"
load X num 65535    # Number 65535 will be loaded into the X register. Would be the same as "lit 0xFFFF"
```
</details>

### ```str``` <a name="str"></a>
<details open>
  <Summary> Explanation </Summary>
  
```str``` is currently only used for ```draw```ing and will simply convert each character into a u16 that will be stored into the GPU buffer without interruption. The assembler will automatically add an escape character ("``` ` ```") to the end of the string so the GPU knows when to exit drawing mode. Whitespace is not allowed inside a ```str```, use the character ```^``` instead. Example:
```ruby
draw str Hello^World!  # Will print "Hello World!" to the screen
```
</details>

### ```col``` <a name="col"></a>
<details open>
  <Summary> Explanation </Summary>
  
```col``` is currently only used for ```draw```ing. It is placed behind a ```str``` to color it. You can also just not use it, then the assembler will default to making the ```str``` white. Example:
```ruby
draw str Hello^World! col red  # Will print a red "Hello World!" to the screen
```
</details>

## Routines <a name="Routines"></a>
```routine: <RoutineName>``` is used to create a routine. Every line below a ```routine: <RoutineName>``` will be part of that routine, until the keyword ```end``` is encountered. ```end```, as the name implies, marks the end of the routine.
All routines **must** be defined before being used. This example program loads the A register with the value 1 and then runs a loop that
increments the value in the A register by 1 for every iteration until it reaches 10000, returns and halts the CPU:
```ruby
routine: loop
radd A num 1
comp reg A num 10000
juin loop      # Jump to routine "loop" if the value in register A isn't equal to 10000
rtor           # Will be reached when the value in register A is equal to 10000
end

routine: entry
load A num 1
jusr loop      # Jump to routine called "loop"
halt           # Routine "loop" returns here when it encounters the "rtor" instruction
end
```

<details>
  <Summary> Detailed explanation </Summary>

  1. In "entry": Loads the A register with the value 1
  2. In "entry": Jumps to a routine called "loop"
  3. In "loop": Add 1 to the value in the A register
  4. In "loop": Compare the value in the A register to 10000, set the eq_flag if true
  5. In "loop": If the eq_flag is **not** set, jump to a routine called "loop" (itself here), otherwise continue
  6. In "loop": "Return to origin" instruction returns to where the program came from, moving on from there
  7. In "entry": Send the CPU into the halt loop, stopping execution
</details>

#

## Instructions <a name="Instructions"></a>
Now it gets interesting. Instructions are key to make the machine do things, so there are (will be) a lot of them

### ```load``` <a name="load"></a>
<details open>
  <Summary> Explanation </Summary>
  
```load``` is used to load a value into a register. Which register is specified by the first argument, the value by the second. Examples:
```ruby
load A num 7
load X hex H
load Y lit 0x06AF
```
</details>

### ```stor``` <a name="stor"></a>
<details open>
  <Summary> Explanation </Summary>
  
```stor``` is used to store a value from the register specified by the first argument to the address specified in the second argument. Examples:
```ruby
stor A lit 0x56FA  # Stores the value saved in the A register to address 0x56FA (the 22266th address) in the memory
stor A num 22266   # You can also use a number directly
```
</details>

### ```jump``` <a name="jump"></a>
<details open>
  <Summary> Explanation </Summary>
  
```jump``` is used to simply jump to a given address. Examples:
```ruby
jump lit 0x56FA    # Jumps to the address 0x56FA (the 22266th address) in the memory
jump num 22266     # You can also use a number directly
```
</details>

### ```jusr``` <a name="jusr"></a>
<details open>
  <Summary> Explanation </Summary>
  
```jusr``` is used just like ```jump``` with the slight difference that it saves the previous position to the stack, allowing the program to return to the previous position using ```rtor```. Examples:
```ruby
jusr lit 0x56FA    # Jumps to the address 0x56FA (the 22266th address) in the memory
jusr num 22266     # You can also use a number directly
```
</details>

### ```juie``` <a name="juie"></a>
<details open>
  <Summary> Explanation </Summary>
  
```juie``` is used just like ```jump``` with the slight difference that it only jumps to the specified address if the CPU's eq_flag is set. Examples:
```ruby
juie lit 0x56FA    # Jumps to the address 0x56FA (the 22266th address) in the memory if the eq_flag is set
juie num 22266     # You can also use a number directly
```
</details>

### ```juin``` <a name="juin"></a>
<details open>
  <Summary> Explanation </Summary>
  
```juin``` is used just like ```jump``` with the slight difference that it only jumps to the specified address if the CPU's eq_flag is **NOT** set. Examples:
```ruby
juin lit 0x56FA    # Jumps to the address 0x56FA (the 22266th address) in the memory if the eq_flag is NOT set
juin num 22266     # You can also use a number directly
```
</details>

### ```rtor``` <a name="rtor"></a>
<details open>
  <Summary> Explanation </Summary>
  
```rtor``` is used to return from a subroutine. Example:
```ruby
rtor    # This doesn't take any arguments
```
</details>

### ```noop``` <a name="noop"></a>
<details open>
  <Summary> Explanation </Summary>
  
```noop``` Simply makes the CPU do nothing for one cycle. Example:
```ruby
noop    # Makes the CPU do nothing for one cycle
```
</details>

### ```setv``` <a name="setv"></a>
<details open>
  <Summary> Explanation </Summary>
  
```setv``` is used to set an address in the memory to the specified value. Examples:
```ruby
setv lit 0x56FA hex U  # Sets the address 0x56FA (the 22266th address) in the memory to the ASCII representation of the character 'U'
setv num 22266 lit 0x0055   # You can also use a number or hex values directly
```
</details>

### ```comp``` <a name="comp"></a>
<details open>
  <Summary> Explanation </Summary>
  
```comp``` is used to compare two values. If those values are equal, the CPU's eq_flag will be set. The values to be compared can either be registers or specified directly. Examples:
```ruby
comp lit 0x4000 num 8    # Compares the hexadecimal value 0x4000 with the decimal value 8
comp reg A num 8         # Compares the content of register A with the decimal value 8
comp reg A reg X         # Compares two registers
```
</details>

### ```radd``` <a name="radd"></a>
<details open>
  <Summary> Explanation </Summary>
  
```radd``` is used to increment a register's value by the following value. Examples:
```ruby
radd A num 8      # Increases the value in the A register by 8
radd X hex 12     # Increases the value in the X register by 0x12 (18 in decimal)
```
</details>

### ```rsub``` <a name="rsub"></a>
<details open>
  <Summary> Explanation </Summary>

```rsub``` is used to decrement a register's value by the following value. Examples:
```ruby
rsub A num 8      # Decreases the value in the A register by 8
rsub X hex 12     # Decreases the value in the X register by 0x12 (18 in decimal)
```
</details>

### ```rmul``` <a name="rmul"></a>
<details open>
  <Summary> Explanation </Summary>

```rmul``` is used to multiply a register's value by the following value. Examples:
```ruby
rmul A num 8      # Multiplies the value in the A register by 8
rmul X hex 12     # Multiplies the value in the X register by 0x12 (18 in decimal)
```
</details>

### ```rdiv``` <a name="rdiv"></a>
<details open>
  <Summary> Explanation </Summary>

```rdiv``` is used to divide a register's value by the following value. Examples:
```ruby
rdiv A num 8      # Divides the value in the A register by 8
rdiv X hex 12     # Divides the value in the X register by 0x12 (18 in decimal)
```
</details>
