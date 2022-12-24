pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 48;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

type Register = u8;
type Address = u16;
type Location = u8;
type StackPointer = u16;
type Timer = u8;
type Opcode = u16;

pub struct Emulator {
    pc: Address,
    ram: [Location; RAM_SIZE],
    address_register: Address,
    registers: [Register; NUM_REGS],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    stack_pointer: StackPointer,
    stack: [Address; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: Timer,
    sound_timer: Timer,
}

impl Default for Emulator {
    fn default() -> Self {
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

impl Emulator {
    pub fn new() -> Self {
        let mut emu: Emulator = Default::default();

        // Load sprites into RAM

        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.address_register = 0;
        self.registers = [0; NUM_REGS];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let instruction = self.fetch();
        // Decode
        // Execute
    }

    fn fetch(&mut self) -> Opcode {
        let higher = self.ram[self.pc as usize] as u16;
        let lower = self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        (higher << 8) | lower
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        };
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        };
    }

    fn push(&mut self, addr: Address) {
        self.stack[self.stack_pointer as usize] = addr;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> Address {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }
}
