// NOTE: CPU OPCODES

// --- OpCodes: NoOp
/// OpCode: No operation. Simply doesn't do anything except increasing the instruction pointer
pub const NO_OPERAT: u16 = 0x00A0;

// --- OpCodes: Halt
/// OpCode: Sets the CPU's halt_flag to true
pub const HALT_LOOP: u16 = 0x00AF;

// --- OpCodes: Load into Register ---
/// OpCode: Loads the following value into A register
pub const LOAD_AREG: u16 = 0x00A1;
/// OpCode: Loads the following value into B register
pub const LOAD_BREG: u16 = 0x00A2;
/// OpCode: Loads the following value into C register
pub const LOAD_CREG: u16 = 0x00A3;
/// OpCode: Loads the following value into D register
pub const LOAD_DREG: u16 = 0x00A4;
/// OpCode: Loads the following value into G register
pub const LOAD_GREG: u16 = 0x00A5;

// --- OpCodes: Load into Register ---
/// OpCode: Stores the A register's value to the following address
pub const STOR_AREG: u16 = 0x00B1;
/// OpCode: Stores the B register's value to the following address
pub const STOR_BREG: u16 = 0x00B2;
/// OpCode: Stores the C register's value to the following address
pub const STOR_CREG: u16 = 0x00B3;
/// OpCode: Stores the D register's value to the following address
pub const STOR_DREG: u16 = 0x00B4;
/// OpCode: Stores the G register's value to the following address
pub const STOR_GREG: u16 = 0x00B5;

// --- OpCodes: Jump to Subroutine ---
/// OpCode: Sets the instruction pointer to the value of the following address, jumping there.
///         This also pushes the previous value to the stack, allowing to return to where the
///         program came from using the ```RET_TO_OR``` (Return To Origin) OpCode.
pub const JMP_TO_SR: u16 = 0x00C1;

// --- OpCodes: Jump to following Address ---
/// OpCode: Sets the instruction pointer to the value of the following address, jumping there.
pub const JMP_TO_AD: u16 = 0x00C0;

// --- OpCodes: Return from Subroutine / Return to Origin ---
/// OpCode: Fetches the value previously pushed to the stack and sets the instruction pointer to
///         it, returning to where the program came from.
pub const RET_TO_OR: u16 = 0x00D1;

// --- OpCodes: Compare two registers ---
/// OpCodes: Compares two registers and sets the eq_flag accordingly.
pub const COMP_REGS: u16 = 0x00D6;

// --- OpCodes: Jump if equal ---
/// OpCodes: Jumps to the following address if the eq_flag is true
pub const JUMP_IFEQ: u16 = 0x00C2;

// --- OpCodes: Jump if not equal ---
/// OpCodes: Jumps to the following address if the eq_flag is false
pub const JUMP_INEQ: u16 = 0x00C3;

// --- OpCodes: Increment register value ---
/// OpCodes: Increases the value in the register specified in the following address by the value
///          specified in the second address after the opcode
pub const INC_REG_V: u16 = 0x00D2;

// --- OpCodes: Decrement register value ---
/// OpCodes: Decreases the value in the register specified in the following address by the value
///          specified in the second address after the opcode
pub const DEC_REG_V: u16 = 0x00D3;

// --- OpCodes: Multiply register value ---
/// OpCodes: Multiplies the value in the register specified in the following address by the value
///          specified in the second address after the opcode
pub const MUL_REG_V: u16 = 0x00D4;

// --- OpCodes: Divide register value ---
/// OpCodes: Divides the value in the register specified in the following address by the value
///          specified in the second address after the opcode
pub const DIV_REG_V: u16 = 0x00D5;

// NOTE: GPU OPCODES
// TODO:

// --- OpCodes: NoOP ---
/// GPU OpCode: This simply makes the GPU do nothing
pub const GPU_NO_OPERAT: u16 = 0xA000;

// --- OpCodes: Draw Letter ---
/// GPU OpCode: Treats the following value as a char, attempts to convert it to ASCII and draw it to the
///             screen, automatically moving the cursor. If the value is invalid, it will output
///             a medium shade ('▒') character.
pub const GPU_DRAW_TEXT: u16 = 0xA001;

// --- OpCodes: Draw Value ---
/// GPU OpCode: Treats the following value as an integer, attempts to convert it to ASCII and draw it to the
///             screen, automatically moving the cursor. If the value is invalid, it will output
///             a medium shade ('▒') character.
pub const GPU_DRAW_VALU: u16 = 0xA003;

// --- OpCodes: Reset Buf Ptr ---
/// GPU OpCode: Resets the GPU's buf_ptr to the beginning of the GPU buffer.
pub const GPU_RESET_PTR: u16 = 0xA0A2;

// --- OpCodes: Update GPU ---
/// GPU OpCode: This will make the GPU redraw the frame buffer
pub const GPU_UPDATE: u16 = 0xA002;

// --- OpCodes: Reset Frame Buffer ---
/// GPU OpCode: This clears the GPU's frame buffer
pub const GPU_RES_F_BUF: u16 = 0xA0A3;

// --- OpCodes: Move the cursor up ---
/// GPU OpCode: This moves the GPU's cursor up one line
pub const GPU_MV_C_UP: u16 = 0xA0B0;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_MV_C_DOWN: u16 = 0xA0B1;

// --- OpCodes: Move the cursor left ---
/// GPU OpCode: This moves the GPU's cursor left one collumn
pub const GPU_MV_C_LEFT: u16 = 0xA0B2;

// --- OpCodes: Move the cursor right ---
/// GPU OpCode: This moves the GPU's cursor right one collumn
pub const GPU_MV_C_RIGH: u16 = 0xA0B3;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves inserts a new line (moves the GPU's cursor down and to the leftmost position)
pub const GPU_NEW_LINE: u16 = 0xA0B4;

