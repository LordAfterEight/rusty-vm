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
