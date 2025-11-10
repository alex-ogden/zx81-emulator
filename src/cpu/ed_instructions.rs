use super::Cpu;
use crate::memory::Memory;
use crate::tape::Tape;

impl Cpu {
    pub(super) fn execute_ed_instruction(
        &mut self,
        opcode: u8,
        memory: &mut Memory,
        io: &mut crate::io::IoController,
        tape: &Option<Tape>,
    ) -> u8 {
        match opcode {
            0x4F => self.ld_r_a(),
            0x47 => self.ld_i_a(),
            0x5F => self.ld_a_r(),
            0x56 => self.im_1(),
            0x78 => self.in_a_c(io, tape),
            0x44 => self.neg(),

            // Consolidated patterns:
            0x4B | 0x5B | 0x7B => self.ld_rr_nn_indirect(opcode, memory),
            0x43 | 0x53 | 0x63 | 0x73 => self.ld_nn_indirect_rr(opcode, memory),
            0x42 | 0x52 | 0x62 | 0x72 => self.sbc_hl_rr(opcode),
            0x4A | 0x5A | 0x6A | 0x7A => self.adc_hl_rr(opcode),

            0xA0 => self.ldi(memory),
            0xB0 => self.ldir(memory),
            0xB1 => self.cpir(memory),
            0xB8 => self.lddr(memory),

            // Tape load/save hooks
            0xFC => self.load_hook(memory, tape),
            0xFD => self.save_hook(memory),

            _ => {
                eprintln!(
                    "Unknown ED opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 2
                );
                4
            }
        }
    }

    fn ldi(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));

        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.bc() != 0);

        16
    }

    fn ldir(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    fn cpir(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_flag_z(self.a == val);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_pv(self.bc() != 0);

        // Check if we need to repeat
        if self.bc() != 0 && self.a != val {
            self.pc = self.pc.wrapping_sub(2); // Go back 2 bytes (ED + opcode)
            return 21;
        }

        16
    }
    fn lddr(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_de(self.de().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    fn ld_rr_nn_indirect(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        let val = memory.read_word(addr);

        match (opcode >> 4) & 0x03 {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => unreachable!(),
        }

        20
    }
    fn ld_nn_indirect_rr(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let val = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        memory.write_word(addr, val);
        20
    }
    fn sbc_hl_rr(&mut self, opcode: u8) -> u8 {
        let hl = self.hl();

        let operand = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        let carry = if self.get_flag_c() { 1u16 } else { 0u16 };
        let result = hl.wrapping_sub(operand).wrapping_sub(carry);

        // Calculate flags
        let full_sub = (hl as u32)
            .wrapping_sub(operand as u32)
            .wrapping_sub(carry as u32);

        self.set_hl(result);
        self.set_flag_c(full_sub > 0xFFFF);
        self.set_flag_n(true);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x8000) != 0);
        self.set_flag_h(((hl & 0x0FFF) as i32 - (operand & 0x0FFF) as i32 - carry as i32) < 0);
        self.set_flag_pv(((hl ^ operand) & (hl ^ result) & 0x8000) != 0);
        self.set_flag_x(((result >> 8) & 0x20) != 0);
        self.set_flag_y(((result >> 8) & 0x08) != 0);

        15
    }

    fn ld_r_a(&mut self) -> u8 {
        self.r = self.a;
        9
    }

    fn ld_i_a(&mut self) -> u8 {
        self.i = self.a;
        9
    }

    fn ld_a_r(&mut self) -> u8 {
        self.a = self.r;
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.iff2);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        9
    }

    fn in_a_c(&mut self, io: &mut crate::io::IoController, tape: &Option<Tape>) -> u8 {
        self.a = io.read_port(self.c, self.b, tape);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        12
    }

    fn im_1(&mut self) -> u8 {
        self.interrupt_mode = 1;
        8
    }

    fn adc_hl_rr(&mut self, opcode: u8) -> u8 {
        let rr = match opcode {
            0x4A => self.bc(),
            0x5A => self.de(),
            0x6A => self.hl(),
            0x7A => self.sp,
            _ => unreachable!(),
        };

        let hl = self.hl();
        let carry = if self.get_flag_c() { 1 } else { 0 };
        let result = hl.wrapping_add(rr).wrapping_add(carry);
        self.set_hl(result);

        self.set_flag_s((result & 0x8000) != 0);
        self.set_flag_z(result == 0);
        self.set_flag_h(((hl & 0x0FFF) + (rr & 0x0FFF) + carry) > 0x0FFF);
        self.set_flag_pv(((hl ^ rr) & 0x8000) == 0 && ((hl ^ result) & 0x8000) != 0);
        self.set_flag_n(false);
        self.set_flag_c((hl as u32 + rr as u32 + carry as u32) > 0xFFFF);
        self.set_flag_x((result & 0x2000) != 0);
        self.set_flag_y((result & 0x0800) != 0);

        15
    }

    fn neg(&mut self) -> u8 {
        let a = self.a;
        let result = 0u8.wrapping_sub(a);

        self.a = result;

        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_z(result == 0);
        self.set_flag_h((0 & 0x0F) < (a & 0x0F));
        self.set_flag_pv(a == 0x80);
        self.set_flag_n(true);
        self.set_flag_c(a != 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        8
    }

    fn load_hook(&mut self, memory: &mut Memory, tape: &Option<Tape>) -> u8 {
        // HL contains the address of the filename (or >= 0x8000 for LOAD "")
        let hl = self.hl();

        println!("LOAD hook triggered: HL=0x{:04X}", hl);

        if let Some(t) = tape {
            // Copy tape data into memory starting at 0x4009
            let start_addr = 0x4009u16;

            println!("Loading {} bytes from tape into memory at 0x{:04X}", t.data.len(), start_addr);

            for (i, &byte) in t.data.iter().enumerate() {
                let addr = start_addr.wrapping_add(i as u16);
                if addr >= 0x8000 {
                    break;
                }
                memory.write(addr, byte);
            }

            // Now find and set up system variables by scanning the loaded data
            let end = start_addr.wrapping_add(t.data.len() as u16);

            println!("Scanning loaded data from 0x{:04X} to 0x{:04X}", start_addr, end);

            // Dump first 32 bytes to see what we loaded
            print!("First 32 bytes: ");
            for i in 0..32 {
                print!("{:02X} ", memory.read(start_addr + i));
            }
            println!();

            // Find D_FILE by looking for a run of consecutive 0x76 bytes (collapsed display)
            // A collapsed display has 24+ consecutive newlines
            let mut addr = start_addr;
            let mut d_file = start_addr;
            let mut consecutive_76 = 0;

            while addr < end {
                if memory.read(addr) == 0x76 {
                    consecutive_76 += 1;
                    if consecutive_76 >= 24 {
                        // Found the display file! It starts where the run began
                        d_file = addr - 23;
                        println!("Found D_FILE at 0x{:04X} (24+ consecutive 0x76 bytes)", d_file);
                        break;
                    }
                } else {
                    consecutive_76 = 0;
                }
                addr = addr.wrapping_add(1);
            }

            if d_file == start_addr {
                println!("WARNING: Could not find D_FILE! Using default location");
                d_file = end.wrapping_sub(32); // Guess: last 32 bytes
            }

            // E_LINE is just before D_FILE
            let e_line = if d_file > start_addr { d_file.wrapping_sub(1) } else { start_addr };

            // Find VARS by continuing through the display file
            // Count 24-25 newlines for the full display
            let mut newline_count = 0;
            addr = d_file;
            while addr < end && newline_count < 25 {
                if memory.read(addr) == 0x76 {
                    newline_count += 1;
                }
                addr = addr.wrapping_add(1);
            }
            let vars = addr;

            // Set all the system variables
            memory.write_word(0x4014, e_line);      // E_LINE
            memory.write_word(0x400C, d_file);      // D_FILE
            memory.write_word(0x4010, vars);        // VARS

            // Set other important system variables
            memory.write_word(0x4016, vars);        // CH_ADD? Not sure
            memory.write_word(0x401A, end);         // STKBOT (bottom of stack)
            memory.write_word(0x401C, end);         // STKEND (end of stack)

            println!("System variables set:");
            println!("  E_LINE  = 0x{:04X}", e_line);
            println!("  D_FILE  = 0x{:04X}", d_file);
            println!("  VARS    = 0x{:04X}", vars);
            println!("  STKEND  = 0x{:04X}", end);

            // Show what's at D_FILE
            print!("D_FILE contents (first 32 bytes): ");
            for i in 0..32 {
                print!("{:02X} ", memory.read(d_file + i));
            }
            println!();

            // Clear carry flag to indicate success
            self.set_flag_c(false);
        } else {
            println!("No tape loaded!");
            // Set carry flag to indicate error
            self.set_flag_c(true);
        }

        4
    }

    fn save_hook(&mut self, _memory: &mut Memory) -> u8 {
        // For now, just acknowledge the save attempt
        let hl = self.hl();
        println!("SAVE hook triggered: HL=0x{:04X} (not implemented)", hl);

        // Clear carry to indicate success (even though we don't actually save)
        self.set_flag_c(false);

        4
    }
}
