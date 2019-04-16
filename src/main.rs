use std::env;
use std::fs::{File,OpenOptions};
use std::process::exit;
use std::io::Write;
use std::thread::sleep;
use std::time;
use std::io;


fn display(data :&Vec<u8>) {
    for i in 0..data.len() {
        if i % 8 == 0 {
            print!("\n");
        }
        print!("{:3}  ", data[i]);
    }
    print!("\n");

}

fn write(fp: &mut File, data: &Vec<u8>) {
    fp.write_all(&data).unwrap();
}

fn prompt(prompt: &str) {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();
}

fn main() {
    let orig_data = vec![
    0x06, 0xbe, 0x15, 0x00, 0x01, 0x01, 0x03, 0x01,  0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    0xff, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print!("Required arg: path to hidraw device\n");
        exit(1);
    }

    let device = &args[1];

    let mut fp = OpenOptions::new().write(true).open(device).expect("Open device failed");

    let mut data = orig_data.clone();
    display(&data);

    write(&mut fp, &data);

    print!("Enter 'h' for help\n");
    loop {
        let mut cmd = String::new();

        prompt(">");

        io::stdin().read_line(&mut cmd).expect("Failed to read command\n");
        let cmd = cmd.trim();
        
        if cmd == "r" {
            let mut data = orig_data.clone();
            display(&data);
        } else if cmd == "h" {
            print!("(p)rint, (r)eset, (w)rite, (q)uit or 0-31 to set values\n");
        } else if cmd == "p" {
            display(&data);
        } else if cmd == "q" {
            break;
        } else if cmd == "w" {
            write(&mut fp, &data);
        } else if cmd == "" {
            
        } else if cmd.chars().all(char::is_numeric) {
            let off : u8 = cmd.parse().expect("Please type a number 0-31\n");
            if off > 31 {
                print!("Please enter an offset, 0-31\n");
            } else {
                prompt("?");
                let mut newstr: String = String::new();
                io::stdin().read_line(&mut newstr).expect("Failed to read command\n");
                let newval: u8 = newstr.trim().parse().expect("Please enter a number\n");
                data[usize::from(off)] = newval;
            }
        }

    }

}
