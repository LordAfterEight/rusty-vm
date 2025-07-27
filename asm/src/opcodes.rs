// NOTE: CPU OPCODES

// --- OpCodes: NoOp
/// OpCode: No operation. Simply doesn't do anything except increasing the instruction pointer
pub const NO_OPERAT: u16 = 0x0000;

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

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_MV_C_UP: u16 = 0xA0B0;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_MV_C_DOWN: u16 = 0xA0B1;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_MV_C_LEFT: u16 = 0xA0B2;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_MV_C_RIGH: u16 = 0xA0B3;

// --- OpCodes: Move the cursor down ---
/// GPU OpCode: This moves the GPU's cursor down one line
pub const GPU_NEW_LINE: u16 = 0xA0B4;

