// --- OpCodes: Load into Register ---
/// OpCode: Loads the following value into A register
pub const LOAD_AREG: u16 = 0x0001;
/// OpCode: Loads the following value into X register
pub const LOAD_XREG: u16 = 0x0002;
/// OpCode: Loads the following value into Y register
pub const LOAD_YREG: u16 = 0x0003;

// --- OpCodes: Store From Register to Following Address ---
/// OpCode: Stores the value from the A register to the following address
pub const STOR_AREG: u16 = 0x0011;
/// OpCode: Stores the value from the X register to the following address
pub const STOR_XREG: u16 = 0x0012;
/// OpCode: Stores the value from the Y register to the following address
pub const STOR_YREG: u16 = 0x0013;

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
