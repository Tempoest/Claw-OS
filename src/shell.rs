use lazy_static::lazy_static;
use spin::Mutex;

use crate::vga_buffer::Color;
use crate::{interrupts, print, print_color, println, println_color, vga_buffer};

const MAX_LINE: usize = 128;

struct LineBuffer {
    bytes: [u8; MAX_LINE],
    len: usize,
}

lazy_static! {
    static ref LINE: Mutex<LineBuffer> = Mutex::new(LineBuffer {
        bytes: [0; MAX_LINE],
        len: 0,
    });
    static ref READY: Mutex<Option<(usize, [u8; MAX_LINE])>> = Mutex::new(None);
}

pub fn handle_char(c: char) {
    let mut line = LINE.lock();
    match c {
        '\n' | '\r' => {
            print!("\n");
            let mut ready = READY.lock();
            if ready.is_none() {
                *ready = Some((line.len, line.bytes));
            }
            line.len = 0;
        }
        '\u{8}' => {
            if line.len > 0 {
                line.len -= 1;
                vga_buffer::backspace();
            }
        }
        c if c.is_ascii_graphic() || c == ' ' => {
            if line.len < MAX_LINE {
                let idx = line.len;
                line.bytes[idx] = c as u8;
                line.len += 1;
                print!("{}", c);
            }
        }
        _ => {}
    }
}

pub fn run() -> ! {
    vga_buffer::enable_cursor();

    println_color!(Color::LightGreen, Color::Black, "Claw OS");
    println!("type `help` for commands\n");
    print_color!(Color::Green, Color::Black, "> ");

    loop {
        let taken = READY.lock().take();
        if let Some((len, bytes)) = taken {
            let line = core::str::from_utf8(&bytes[..len]).unwrap_or("");
            execute(line.trim());
            print_color!(Color::Green, Color::Black, "> ");
        } else {
            x86_64::instructions::hlt();
        }
    }
}

fn execute(line: &str) {
    match line {
        "" => {}
        "help" => println!("commands: help, clear, uptime, echo <text>, reboot"),
        "clear" => vga_buffer::clear(),
        "uptime" => println_color!(
            Color::LightCyan,
            Color::Black,
            "up {} s",
            interrupts::ticks() / 1000
        ),
        "reboot" => unsafe {
            x86_64::instructions::port::Port::new(0x64).write(0xFEu8);
        },
        _ => match line.strip_prefix("echo ") {
            Some(rest) => println!("{}", rest),
            None => println_color!(Color::LightRed, Color::Black, "unknown command: {}", line),
        },
    }
}
