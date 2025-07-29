# Rusty-VM
Rusty-VM is a 16-bit virtual machine with completely made up hardware, architecture, instructions, you name it. It has its own Assembly-ish language called RvmASM and comes with an assembler that takes .rvmasm files
and turns them into a memory file with binary content for the VM to read and write to. The goal is to have a fully functional 16-bit virtual machine that can run any program or even OS made for it using RvmASM

### The VM is currently under development and far from being finished. Most of what you read here is probably not implemented yet.
|State           |  Symbol|
|----------------|--------|
|Implemented     |  ✅    |
|Unfinished/WIP  |  🟡    |
|Missing         |  ❌    |

# Parts / Modules
The VM consists of (currently) three key parts: the CPU, the GPU and the memory
- The CPU reads instructions from the memory, executes them and manages the GPU. It also writes to the memory, primarily to the GPU buffer to control what the GPU does
- The GPU has its own personal space in the memory, called the GPU Buffer. It spans from address ```0x0300``` to address ```0x04FF```, so 512 16-bit spaces or 1024B
- The memory is basically the communication layer between the CPU and the GPU and at the same time that makes both able to do literally anything. It's 128kB in size (65536 16-bit addresses) and certain regions are always preprogrammed

<details>
  <Summary> Memory Layout </Summary>
  
  The memory has a few regions preprogrammed with information. The following table shows the memory layout
  | Region          | Stored Data                                  |
  |-----------------|----------------------------------------------|
  | ```0x0000 - 0x01FF``` | Currently unused                             |
  | ```0x0200 - 0x0250``` | ASCII buffer with stored letters and symbols |
  | ```0x0251 - 0x02FF``` | Currently unused                             |
  | ```0x0300 - 0x04FF``` | GPU buffer                                   |
  | ```0x0500 - 0xFFFF``` | Empty space, used for programs               |

</details>

# Specs
<details>
  <Summary> CPU 🟡 </Summary>
  <details>
    <Summary> Registers ✅ </Summary>
    
    | Register | Purpose                                                              |
    |----------|----------------------------------------------------------------------|
    | A        | 16-bit general purpose register                                      |
    | X        | 16-bit general purpose register                                      |
    | Y        | 16-bit general purpose register                                      |
    | G        | 16-bit register used for CPU-GPU interaction, not accessible in code |

  </details>
  <details>
    <Summary> OpCodes 🟡 </Summary>
    Right now there is a total of 30 OpCodes
    For a detailed documentation see the [RvmASM documentation](https://github.com/LordAfterEight/rusty-vm/tree/master/asm).
    
  </details>

</details>

#

<details>
  <Summary> GPU </Summary>
  A GPU with a 91x49 characters framebuffer, capable of printing colored text.
</details>
