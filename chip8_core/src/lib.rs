use rand::Rng;

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
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.clear_screen();
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let instruction = self.fetch();
        // Decode and Execute
        self.execute(instruction);
    }

    fn fetch(&mut self) -> Opcode {
        let higher = self.ram[self.pc as usize] as u16;
        let lower = self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        (higher << 8) | lower
    }

    fn execute(&mut self, instruction: Opcode) {
        // Match by digit
        let digit3 = (instruction >> 12) & 0xF;
        let digit2 = (instruction >> 8) & 0xF;
        let digit1 = (instruction >> 4) & 0xF;
        let digit0 = instruction & 0xF;

        match (digit3, digit2, digit1, digit0) {
            (0, 0, 0, 0) => (),
            (0, 0, 0xE, 0) => self.clear_screen(),
            (0, 0, 0xE, 0xE) => self.pc = self.pop(),
            (1, _, _, _) => self.pc = instruction & 0xFFF,
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = instruction & 0xFFF;
            },
            (3, register, _, _) => {
                let nn = instruction & 0xFF;
                if self.registers[register as usize] as Address == nn {
                    self.pc += 2; // Skip one opcode
                }
            },
            (4, register, _, _) => {
                let nn = instruction & 0xFF;
                if self.registers[register as usize] as Address != nn {
                    self.pc += 2; // Skip one opcode
                }
            },
            (5, x, y, 0) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc += 2; // Skip one opcode
                }
            },
            (6, register, _, _) => {
                let nn = instruction & 0xFF;
                self.registers[register as usize] = nn as Register;
            },
            (7, register, _, _) => {
                // Doesn't affect the carry flag 
                let nn = instruction & 0xFF;
                let sum = self.registers[register as usize].wrapping_add(nn as Register);
                self.registers[register as usize] = sum;
            },
            (8, x, y, 0) => {
                self.registers[x as usize] = self.registers[y as usize];
            },
            (8, x, y, 1) => {
                self.registers[x as usize] |= self.registers[y as usize];
            },
            (8, x, y, 2) => {
                self.registers[x as usize] &= self.registers[y as usize];
            },
            (8, x, y, 3) => {
                self.registers[x as usize] ^= self.registers[y as usize];
            },
            (8, x, y, 4) => {
                let (sum, carry) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = sum;
                self.registers[0xF] = if carry { 1 } else { 0 };
            },
            (8, x, y, 5) => {
                let (sub, borrow) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = sub;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            },
            (8, x, _, 6) => {
                let dropped_bit = self.registers[x as usize] & 0x1;
                self.registers[x as usize] >>= 1;
                self.registers[0xF] = dropped_bit;
            },
            (8, x, y, 7) => {
                let (sub, borrow) = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                self.registers[x as usize] = sub;
                self.registers[0xF] = if borrow { 0 } else { 1 };
            },
            (8, x, _, 0xE) => {
                let dropped_bit = self.registers[x as usize] >> 7 & 0x1;
                self.registers[x as usize] <<= 1;
                self.registers[0xF] = dropped_bit;
            },
            (9, x, y, 0) => {
                if self.registers[x as usize] != self.registers[y as usize] { self.pc += 2 };
            },
            (0xA, _, _, _) => {
                self.address_register = instruction & 0xFFF;
            },
            (0xB, _, _, _) => {
                self.pc = self.registers[0] as Address + (instruction & 0xFFF);
            },
            (0xC, x, _, _) => {
                let nn = (instruction & 0xFF) as u8;
                self.registers[x as usize] = self.rand() & nn;
            },
            (0xD, x, y, sprite_height) => {
                let x_coord = self.registers[x as usize] as usize;
                let y_coord = self.registers[y as usize] as usize;
                let start = self.address_register as usize;

                let mut flipped = false;

                for row  in 0..(sprite_height as usize) {
                    let pixels = self.ram[start + row];
                    for digit in 0..8 {
                        let pixel = pixels & 0x80 >> digit;
                        // It doesn't clear the screen so we only flip if it's a 1
                        if pixel != 0 {
                            // Sprites wrap around the screen
                            let x = (x_coord + digit) % SCREEN_WIDTH;
                            let y = (y_coord + row) % SCREEN_HEIGHT;
                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.display[idx];

                            // Flip
                            self.display[idx] ^= true;
                        }
                    }
                }

                self.registers[0xF] = if flipped { 1 } else { 0 };
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}!", instruction),
        }
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

    fn rand(&self) -> Register {
        rand::thread_rng().gen()
    }

    fn clear_screen(&mut self) {
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
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
