use std::io::{self, Write};
use std::process::{self, Command, exit};

pub struct Wish;

enum BuiltinCommands
{
    execute,
    path,
    cd,
    exit,
}

//일단 대화형 우선 개발

// 1 step : execute
// 2 step : path
// 3 step : cd (execute, path 수정 필요)
// 4 step : Parallel Commands

impl Wish {
    pub fn run() {
        loop {
            print!("wish> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                }
                Err(error) => {
                    println!("error");
                    panic!()
                },
            }

            let command = input.trim_end();
            match Self::commandFromString(&command) {
                BuiltinCommands::exit => {break},
                BuiltinCommands::cd => { println!("cd"); },
                BuiltinCommands::path => { println!("path");},
                BuiltinCommands::execute => {Self::execute_program(&command)},
                _ => { println!("{}", input); panic!() }
            }
        }
    }

    fn execute_program(file_name : &str){
        // 외부 프로그램 실행
        Command::new(file_name)
            .spawn()
            .unwrap()
            .wait()
            .expect("Failed to execute command");
    }


    fn commandFromString(input_command : &str) -> BuiltinCommands {
        match input_command {
            "path" => BuiltinCommands::path,
            "cd" => BuiltinCommands::cd,
            "exit" => BuiltinCommands::exit,
            _ => BuiltinCommands::execute,
        }
    }

    // fn StringToCommands(input_string : String){
    //     input_string.split(' ').collect()
    // }
}