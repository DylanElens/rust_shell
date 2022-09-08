use std::io::{stdin, stdout, Write};
use std::{process::Command, result};

#[derive(Debug)]
struct UserCommand {
    args: Vec<String>,
}

#[derive(Debug)]
struct Expression {
    commands: Vec<UserCommand>,
    input_from_file: Option<String>,
    output_to_file: Option<String>,
    background: bool,
}

fn parseToExpression(s: &str) -> Expression {
    let mut commands = Vec::new();
    let mut input_from_file = None;
    let mut output_to_file = None;
    let mut background = false;
    let mut current_command = UserCommand { args: Vec::new() };
    let mut in_quotes = false;
    for c in s.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == '|' && !in_quotes {
            commands.push(current_command);
            current_command = UserCommand { args: Vec::new() };
        } else if c == '<' && !in_quotes {
            input_from_file = Some(String::new());
        } else if c == '>' && !in_quotes {
            output_to_file = Some(String::new());
        } else if c == '&' && !in_quotes {
            background = true;
        } else if c == ' ' && !in_quotes {
            if input_from_file.is_some() {
                input_from_file = Some(current_command.args.join(" "));
                current_command.args = Vec::new();
            } else if output_to_file.is_some() {
                output_to_file = Some(current_command.args.join(" "));
                current_command.args = Vec::new();
            } else {
                if current_command.args.len() > 0 {
                    current_command.args.push(String::new());
                }
            }
        } else {
            if input_from_file.is_some() {
                input_from_file.as_mut().unwrap().push(c);
            } else if output_to_file.is_some() {
                output_to_file.as_mut().unwrap().push(c);
            } else {
                if current_command.args.len() == 0 {
                    current_command.args.push(String::new());
                }
                current_command.args.last_mut().unwrap().push(c);
            }
        }
    }
    if current_command.args.len() > 0 {
        commands.push(current_command);
    }
    return Expression {
        commands,
        input_from_file,
        output_to_file,
        background,
    };
}

fn executeExpression(expression: &Expression) {
    let mut result: result::Result<(), String>;
    for command in &expression.commands {
        if expression.background {
            result = Command::new(&command.args[0])
                .args(&command.args[1..])
                .spawn()
                .map(|_| ())
                .map_err(|e| e.to_string());
        } else {
            println!("{:?}", &command);
            println!("{:?}", &command);
            result = Command::new(&command.args[0].trim_end())
                .args(&command.args[1..].into_iter().map(|arg| arg.trim_end()))
                .status()
                .map(|_| ())
                .map_err(|e| e.to_string());
        }

        match result {
            Ok(_) => {
                println!("Command executed successfully");
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    // TODO
    // 1. Fork
    // 2. Child process: execvp
    // 3. Parent process: waitpid
    // 4. Handle input/output redirection
    // 5. Handle background process
    // 6. Handle pipe
    // 7. Handle error
    // 8. Handle exit
    // 9. Handle cd
}

fn main() {
    let mut input = String::new();
    print!("$ ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    let args: Expression = parseToExpression(&input);
    executeExpression(&args);
}
