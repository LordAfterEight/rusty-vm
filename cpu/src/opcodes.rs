// NOTE: CPU OPCODES

// --- OpCodes: NoOp
/// OpCode: No operation. SImply doesn't do anything except increasing the instruction pointer
pub const NO_OPERAT: u16 = 0x0000;

// --- OpCodes: Halt
/// OpCode: Sets the CPU's halt_flag to true
pub const HALT_LOOP: u16 = 0x000F;

// --- OpCodes: Load into Register ---
/// OpCode: Loads the following value into A register
pub const LOAD_AREG: u16 = 0x0001;
/// OpCode: Loads the following value into X register
pub const LOAD_XREG: u16 = 0x0002;
/// OpCode: Loads the following value into Y register
pub const LOAD_YREG: u16 = 0x0003;

// --- OpCodes: Jump to Subroutine ---
/// OpCode: Sets the instruction pointer to the value of the following address, jumping there.
///         This also pushes the previous value to the stack, allowing to return to where the
///         program came from using the ```RET_TO_OR``` (Return To Origin) OpCode.
pub const JMP_TO_SR: u16 = 0x0021;

// --- OpCodes: Jump to following Address ---
/// OpCode: Sets the instruction pointer to the value of the following address, jumping there.
pub const JMP_TO_AD: u16 = 0x0020;

// --- OpCodes: Return from Subroutine / Return to Origin ---
/// OpCode: Fetches the value previously pushed to the stack and sets the instruction pointer to
///         it, returning to where the program came from.
pub const RET_TO_OR: u16 = 0x0031;

// --- OpCodes: Compare two registers ---
/// OpCodes: Compares two registers and sets the eq_flag accordingly.
pub const COMP_REGS: u16 = 0x0004;

// --- OpCodes: Jump if equal ---
/// OpCodes: Jumps to the following address if the eq_flag is set
pub const JUMP_IFEQ: u16 = 0x0022;

// --- OpCodes: Increment register value ---
/// OpCodes: Increases the value in the register specified in the following address by the value
///          specified in the second address after the opcode
pub const INC_REG_V: u16 = 0x0042;

// NOTE: GPU OPCODES
// TODO:

// --- OpCodes: NoOP ---
/// GPU OpCode: This simply makes the GPU do nothing
pub const GPU_NO_OPERAT: u16 = 0xA000;

// --- OpCodes: Draw Letter ---
/// GPU OpCode: Reads the following value and attempts to convert it to ASCII and draw it to the
///             screen, automatically moving the cursor. If the value is invalid, it will output
///             a medium shade ('▒') character.
pub const GPU_DRAW_LETT: u16 = 0xA001;

// --- OpCodes: Reset Buf Ptr ---
/// GPU OpCode: Resets the GPU's buf_ptr to the beginning of the GPU buffer.
pub const GPU_RESET_PTR: u16 = 0xA0A2;

// --- OpCodes: Update GPU ---
/// GPU OpCode: This sets the value at 0x
pub const GPU_UPDATE: u16 = 0xA002;

// --- OpCodes: Reset Frame Buffer ---
/// GPU OpCode: This clears the GPU's frame buffer
pub const GPU_RES_F_BUF: u16 = 0xA0A3;
