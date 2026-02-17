// yea, this is debugger from Koshka I RS 
// debugger: yes
// bios: no
//use std::{io::{self, Write}, time::{Duration, Instant}};
//
//use crate::asm::isa::*;
//use crate::cpu::KoshkaCPU2;
//use crate::video::VideoController2;
//use crate::asm::asm::KRSAssembler;
//use crate::k_epoch::k_epoch::KEpoch;
//
//
//pub struct KoshkaDB;
//
//impl KoshkaDB {
//    pub fn shell(cpu: &mut crate::cpu::KoshkaCPU) {
//        loop {
//            print!("~ ");
//            io::stdout().flush().unwrap();
//            
//            let mut input = String::new();
//            io::stdin().read_line(&mut input).unwrap();
//            
//            let mut parts: std::str::SplitWhitespace<'_> = input.trim().split_whitespace();
//            let command: &str = parts.next().unwrap_or("");
//            let args: Vec<&str> = parts.collect();
//            
//            match command {
//                "" => (),
//                "h" => {
//                    VideoController2::disp("Available Commands:");
//                    VideoController2::disp("h - Help");
//                    VideoController2::disp("c - Print Koshka I RS CPU state");
//                    VideoController2::disp("m - Print Koshka I RS memory state");
//                    VideoController2::disp("r - Print Koshka I RS registers state");
//                    VideoController2::disp("w - Write to memory: w [address] [value]");
//                    VideoController2::disp("R - Read from memory: R [address]");
//                    VideoController2::disp("C - Clear the screen");
//                    VideoController2::disp("e - Execute a binary code from memory");
//                    VideoController2::disp("l - Load binary data to memory: l filename.bin");
//                    VideoController2::disp("t - Show current K-Epoch state");
//                    VideoController2::disp("q - Quit shell");
//                },
//                "c" => {
//                    cpu.print_state();
//                },
//                "m" => {
//                    let limit = args.get(0)
//                        .and_then(|arg| arg.parse().ok())
//                        .unwrap_or(256);
//
//                    for (i, byte) in cpu.memory.iter().enumerate().take(limit) {
//                        if i % 16 == 0 {
//                            print!("\n{:04X}: ", i);
//                        }
//                        print!("{:02X} ", byte);
//                    }
//                    println!();
//                },
//                "r" => { 
//                    for i in 0..16 {
//                        print!("K{}: {:02X}     ", i, cpu.k[i]);
//                        if i == 7 || i == 15 {
//                            println!();
//                        }
//                    }
//                    println!();
//                    println!("KADV1: {}", cpu.kadv);
//                    println!("KADV2: {}", cpu.kadv2);
//                },
//                "w" => {
//                    if let Some(addr_str) = args.get(0) {
//                        if let Some(value_str) = args.get(1) {
//                            if let Ok(addr) = u32::from_str_radix(addr_str, 16) {  
//                                if let Ok(value) = u32::from_str_radix(value_str, 16) {
//                                    cpu.write(addr.into(), value); 
//                                    VideoController2::disp(&format!("Written 0x{:02X} to 0x{:02X}", value, addr));
//                                } else {
//                                    VideoController2::disp("Invalid value (use hex)");
//                                }
//                            } else {
//                                VideoController2::disp("Invalid address (use hex)");
//                            }
//                        } else {
//                            VideoController2::disp("Usage: w <addr> <value> (both hex)");
//                        }
//                    } else {
//                        VideoController2::disp("Usage: w <addr> <value>");
//                    }
//                },
//                "R" => {
//                    if let Some(addr_str) = args.get(0) {
//                        if let Ok(addr) = u8::from_str_radix(addr_str, 16) {
//                            let value = cpu.read(addr); 
//                            VideoController2::disp(&format!("0x{:04X}: 0x{:02X}", addr, value));
//                        } else {
//                            VideoController2::disp("Invalid address (use hex)");
//                        }   
//                    } else {
//                        VideoController2::disp("Usage: R <addr>");
//                    }
//                },
//                "C" => {
//                    print!("\x1B[2J\x1B[1;1H");
//                    io::stdout().flush().unwrap();
//                },
//                //"e" => {
//                //    cpu.pc = 0;
//                //    let start_time: Instant = std::time::Instant::now();
//                //    let max_duration: Duration = std::time::Duration::from_secs(1); 
//
//                //    loop {
//                //        KRSAssembler::exec(cpu, cpu.verbose, ia);
//                //        if cpu.memory[cpu.pc as usize] == 0xFF { // HLT
//                //            break;
//                //        }
//                          // if cpu.memory[cpu.pc as usize] == 0x00 && cpu.memory[cpu.pc as usize + 1] == 0x00 {
//                          //     println!("[WARN] Infinite NOP loop detected at {:04X}, stopping(not panicing)", cpu.pc);
//                          //     break;
//                          // }
//                //    }
//                //}
//                "l" => {
//                    if let Some(filename) = args.get(0) {
//                        if let Err(e) = KRSAssembler::load_from_bin(cpu, filename) {
//                            VideoController2::disp(&format!("Error loading BIN file: {}", e));
//                        } else {
//                            VideoController2::disp("BIN file loaded successfully");
//                        }
//                    } else {
//                        VideoController2::disp("Usage: l <filename>.bin");
//                    }
//                },
//                // "t" => {
//                //     VideoController2::disp(&format!("The time is: {}", KEpoch::format(&epoch)));
//                // },
//                "q" => {
//                    break;
//                },   
//                _ => {
//                    VideoController2::disp("Unknown command. Type 'h' for command list.");
//                }
//            } 
//        }
//    }
//}
//