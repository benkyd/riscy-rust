use std::io::{stdin, Write};

use clap::Parser;

#[derive(Debug)]
pub struct VMAction {
    pub action: Action,
    pub arg: String,
}

#[derive(Debug)]
pub enum Action {
    Load,
    Step,
    Run,
    Inspect,
    Dump,
    Reg,
    Quit,
}

pub struct Management {
    pause: bool,
}

#[derive(Parser, Debug)]
#[command(name = "Riscy-Rust, a RV32ima Virtual Machine")]
#[command(version = "0.1.0")]
#[command(author = "Ben Kyd <benjaminkyd@gmail>")]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    load: Option<String>,
    #[arg(short, long)]
    run: bool,
}

impl Management {
    pub fn new() -> Management {
        Management { pause: true }
    }

    pub fn vm_params(&self) -> Vec<VMAction> {
        let cli = Cli::parse();
        let mut actions = Vec::new();

        match cli.load {
            Some(file) => actions.push(VMAction {
                action: Action::Load,
                arg: file,
            }),
            None => (),
        }

        if cli.run {
            actions.push(VMAction {
                action: Action::Run,
                arg: String::new(),
            })
        }
        actions
    }

    pub fn prompt(&self) -> VMAction {
        print!("VM >> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "load" => VMAction {
                action: Action::Load,
                arg: args.next().unwrap().to_string(),
            },
            "step" => VMAction {
                action: Action::Step,
                arg: String::new(),
            },
            "run" => VMAction {
                action: Action::Run,
                arg: String::new(),
            },
            "quit" | "q" => VMAction {
                action: Action::Quit,
                arg: String::new(),
            },
            "help" | "h" => {
                println!("VM > Commands:");
                println!("VM > load <file> - load a program into memory");
                println!("VM > step - step through the program");
                println!("VM > run - run the program");
                println!("VM > quit - quit the program");
                self.prompt()
            }
            _ => {
                println!("VM > Command {} not found", command);
                self.prompt()
            }
        }
    }
}
