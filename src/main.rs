use std::process::Command;

#[derive(Debug)]
struct UserCommand {
    args: Vec<String>,
}

#[derive(Debug)]
struct Expression {
    commands: Vec<UserCommand>,
    operator: String,
    input_from_file: Option<String>,
    output_to_file: Option<String>,
    background: bool,
}

fn split_string_to_args(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    for c in s.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == ' ' && !in_quotes {
            if current_arg.len() > 0 {
                args.push(current_arg);
                current_arg = String::new();
            }
        } else {
            current_arg.push(c);
        }
    }
    if current_arg.len() > 0 {
        args.push(current_arg);
    }
    return args;
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
    // TODO
    // 1. Fork
    for command in &expression.commands {
        let mut child = Command::new(&command.args[0])
            .arg(&command.args[1])
            .spawn()
            .expect("failed to execute child");
        let ecode = child.wait().expect("failed to wait on child");
        print!("child exited with code: {}", ecode);
    }
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
    std::io::stdin().read_line(&mut input).unwrap();
    let args: Expression = parseToExpression(&input);
    executeExpression(&args);
    println!("{:?}", args);
}
