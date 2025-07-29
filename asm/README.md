# RvmASM Assembler
This is the assembler for ```.rvmasm``` files. It will parse any file of that type and convert it into a text file with binary content for [Rusty-VM](https://github.com/lordaftereight/rusty-vm) to read. to use it, first run the following command:
```shell
cargo install --path .
```
Now you can use the ```rvmasm``` command to build a binary from any ```.rvmasm``` input file:
```shell
rvmasm code.rvmasm output
```

# Documentation
RvmASM is an Assembly-ish language for my 16-bit virtual machine Rusty-VM. I made this assembly language and its parser to allow me and maybe even others to easily create programs for the virtual machine without needing to write raw binary values into a file. It is currently under development, just like the virtual machine itself, so both are far from being finished. Under this paragraph you will find a documentation of the entire language. This documentation will constantly change as more features and content are added to the language.

# Table of Contents
1. [Keywords](#Keywords)
   - [lit](#lit)
   - [hex](#hex)
   - [num](#num)
   - [str](#str)
   - [col](#col)
2. [Instructions](#Instructions)
   - [load](#load)
   - [stor](#stor)
   - [jump](#jump)
   - [jusr](#jusr)
   - [juie](#juie)
   - [jine](#jine)
   - [rtor](#rtor)
   - [noop](#noop)
   - [setv](#setv)
   - [comp](#comp)
   - [radd](#radd)
   - [rsub](#rsub)
   - [rmul](#rmul)
   - [rdiv](#rdiv)

## Keywords <a name="Keywords"></a>
These are used to determine what type the following value will be converted to.
There are five keywords: ```lit```, ```hex```, ```num``` and ```str```:


### ```lit``` <a name="lit"></a>
<details open>
  <Summary> Explanation </Summary>
  
```lit``` will return the value as is (thus "lit" for "literal"), which is why the value must not be longer than four characters and it must not contain any special symbols; only ```0-9```and ```A-F``` are allowed. Especially useful when   you need to specify addresses. Examples:
```ruby
lit 0x0FA3
lit 0FA3
# Those two are the same
```
</details>

### ```hex``` <a name="hex"></a>
<details open>
  <Summary> Explanation </Summary>
  
```hex``` will convert the following value into its hexadecimal representation. Examples:
```ruby
load A hex U        # "U" will be converted to 0x0055 and loaded into Register A
load A lit 0x0055   # same as above (without the conversion obviously)
```
</details>

### ```num``` <a name="num"></a>
<details open>
  <Summary> Explanation </Summary>
  
```num``` enables you to use any decimal number from 0 to 65535. Examples:
```ruby
load A num 7        # number 7 will be loaded into the A register. Would be the same as "lit 0x0007"
load X num 65535    # number 65535 will be loaded into the X register. Would be the same as "lit 0xFFFF"
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

### ```jine``` <a name="jine"></a>
<details open>
  <Summary> Explanation </Summary>
  
```jine``` is used just like ```jump``` with the slight difference that it only jumps to the specified address if the CPU's eq_flag is **NOT** set. Examples:
```ruby
jine lit 0x56FA    # Jumps to the address 0x56FA (the 22266th address) in the memory if the eq_flag is NOT set
jine num 22266     # You can also use a number directly
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
