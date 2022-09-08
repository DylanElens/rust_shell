#[derive(Debug)]
struct Command {
    args: Vec<String>,
}

struct Expression {
    commands: Vec<Command>,
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

fn parse_command(s: &str) -> Command {
    let args = split_string_to_args(s);
    let command = Command { args };
    return command;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let args: Command = parse_command(&input);
    println!("{:?}", args);

}
