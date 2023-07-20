use std::io::{stdin, Write};

pub struct VMAction {
    action: Action,
}

pub enum Action {
    Load,
    Step,
    Run,
    Inspect,
    Dump,
    Reg,
}

pub struct Management {
    pause: bool,
}

impl Management {
    pub fn new() -> Management {
        Management { pause: true }
    }

    pub fn vm_params() -> Vec<VMAction> {
        vec![VMAction { action: Action::Load }]
    }

    pub fn prompt(&self) -> Action {
        print!("VM >> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "load" => Action::Load,
            "step" => Action::Step,
            "run" => Action::Run,
            _ => {
                println!("VM > Command {} not found", command);
                self.prompt()
            }
        }
    }
}
