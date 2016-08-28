extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn main() {
    let stdin = 0;
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];

    let mut termios = Termios::from_fd(stdin).unwrap();
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut termios).unwrap();

    loop {
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        println!("You have hit: {:?}", buffer);
    }
}
