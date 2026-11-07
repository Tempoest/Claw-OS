use lazy_static::lazy_static;
use spin::Mutex;

use crate::vga_buffer::Color;
use crate::{interrupts, print, print_color, println, println_color, vga_buffer};

const MAX_LINE: usize = 128;
const QUEUE_LEN: usize = 4;

struct LineBuffer {
    bytes: [u8; MAX_LINE],
    len: usize,
}

struct LineQueue {
    slots: [(usize, [u8; MAX_LINE]); QUEUE_LEN],
    head: usize,
    len: usize,
}

impl LineQueue {
    const fn new() -> Self {
        LineQueue {
            slots: [(0, [0; MAX_LINE]); QUEUE_LEN],
            head: 0,
            len: 0,
        }
    }

    fn push(&mut self, len: usize, bytes: [u8; MAX_LINE]) {
        if self.len == QUEUE_LEN {
            return;
        }
        let idx = (self.head + self.len) % QUEUE_LEN;
        self.slots[idx] = (len, bytes);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<(usize, [u8; MAX_LINE])> {
        if self.len == 0 {
            return None;
        }
        let line = self.slots[self.head];
        self.head = (self.head + 1) % QUEUE_LEN;
        self.len -= 1;
        Some(line)
    }
}

lazy_static! {
    static ref LINE: Mutex<LineBuffer> = Mutex::new(LineBuffer {
        bytes: [0; MAX_LINE],
        len: 0,
    });
    static ref QUEUE: Mutex<LineQueue> = Mutex::new(LineQueue::new());
}

enum Echo {
    Char(char),
    Backspace,
    Newline,
    Nothing,
}

pub fn handle_char(c: char) {
    let (action, finished) = {
        let mut line = LINE.lock();
        match c {
            '\n' | '\r' => {
                let finished = (line.len, line.bytes);
                line.len = 0;
                (Echo::Newline, Some(finished))
            }
            '\u{8}' => {
                if line.len == 0 {
                    return;
                }
                line.len -= 1;
                (Echo::Backspace, None)
            }
            c if (c.is_ascii_graphic() || c == ' ') && line.len < MAX_LINE => {
                let idx = line.len;
                line.bytes[idx] = c as u8;
                line.len += 1;
                (Echo::Char(c), None)
            }
            _ => (Echo::Nothing, None),
        }
    };

    match action {
        Echo::Char(c) => print!("{}", c),
        Echo::Backspace => vga_buffer::backspace(),
        Echo::Newline => print!("\n"),
        Echo::Nothing => {}
    }

    if let Some((len, bytes)) = finished {
        x86_64::instructions::interrupts::without_interrupts(|| QUEUE.lock().push(len, bytes));
    }
}

pub fn run() -> ! {
    vga_buffer::enable_cursor();

    println_color!(Color::LightGreen, Color::Black, "Claw OS");
    println!("type `help` for commands\n");
    prompt();

    loop {
        let next = x86_64::instructions::interrupts::without_interrupts(|| QUEUE.lock().pop());
        match next {
            Some((len, bytes)) => {
                let line = core::str::from_utf8(&bytes[..len]).unwrap_or("");
                execute(line.trim());
                prompt();
            }
            None => x86_64::instructions::interrupts::enable_and_hlt(),
        }
    }
}

fn prompt() {
    print_color!(Color::Green, Color::Black, "> ");
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
