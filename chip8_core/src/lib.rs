use core::time;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 48;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

type Register = u8;
type ProgramCounter = u16;
type AddressRegister = u16;
type Location = u8;
type StackPointer = u16;
type Timer = u8;

pub struct Emulator {
    pc: ProgramCounter,
    ram: [Location; RAM_SIZE],
    address_register: AddressRegister,
    registers: [Register; NUM_REGS],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    stack_pointer: StackPointer,
    stack: [StackPointer; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: Timer,
    sound_timer: Timer,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            address_register: 0,
            registers: [0; NUM_REGS],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
