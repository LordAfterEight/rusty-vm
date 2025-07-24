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
