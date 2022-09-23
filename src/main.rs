use std::io::{stdin, stdout, Read, Write};

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
static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
fn parse_to_expression(s: &str) -> Expression {
    let mut commands = Vec::new();
    let mut input_from_file = None;
    let mut output_to_file = None;
    let mut background = false;
    for command in s.split("|") {
        let mut args = Vec::new();
        for arg in command.split_whitespace() {
            if arg == "<" {
                input_from_file = Some(command.split_whitespace().last().unwrap().to_string());
                break;
            } else if arg == ">" {
                output_to_file = Some(command.split_whitespace().last().unwrap().to_string());
                break;
            } else if arg == "&" {
                background = true;
                break;
            } else {
                args.push(arg.to_string());
            }
        }
        commands.push(UserCommand { args });
    }
    return Expression {
        commands,
        input_from_file,
        output_to_file,
        background,
    };
}
fn execute_expression(expression: &Expression) {
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

    //execute all the commands in another process using libc::fork
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        //child process
        let mut args = Vec::new();
        for command in &expression.commands {
            for arg in &command.args {
                args.push(arg.as_str());
            }
        }

        let mut c_args: Vec<*const libc::c_char> = args.iter().map(|s| s.as_ptr() as *const libc::c_char).collect();
        c_args.push(std::ptr::null());
        unsafe {
            libc::execvp(c_args[0], c_args.as_ptr());
            libc::perror(c_args[0]);
        }
            std::process::exit(1);
    } else {
        //parent process
        unsafe {
            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    }

    // for command in &args {
    //     let mut c_args: Vec<*const i8> = Vec::new();
    //     for arg in command.split_whitespace() {
    //         c_args.push(arg.as_ptr() as *const i8);
    //     }
    //     c_args.push(std::ptr::null() as *const i8);
    //
    //     let pid = unsafe { libc::fork() };
    //     if pid == 0 {
    //         // child process
    //         unsafe {
    //             libc::execvp(c_args[0], c_args.as_mut_ptr());
    //             //print error
    //             libc::perror(c_args[0]);
    //             c_args.clear();
    //         }
    //     } else {
    //         // parent process
    //         let mut status = 0;
    //         unsafe {
    //             libc::waitpid(pid, &mut status, 0);
    //         }
    //     }
    // }
    //
    // let pid = unsafe { libc::fork() };
    // if pid == 0 {
    //     // child process
    //     unsafe {
    //         libc::execvp(c_args[0], c_args.as_mut_ptr());
    //     }
    // } else {
    //     // parent process
    //     let mut status = 0;
    //     unsafe {
    //         libc::waitpid(pid, &mut status, 0);
    //     }
    // }
}

fn main() {
    let mut buffer = String::new();
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();
        let expression = parse_to_expression(&buffer);
        println!("{:?}", expression);
        execute_expression(&expression);
        buffer.clear();
    }
}

// Language: rust
// // Path: Cargo.toml
// [package]
// name = "shell"
// version = "0.1.0"
// authors = ["Rajiv <
//
// fn main() {
//     let mut input = String::new();
//     let current_dir = std::env::current_dir().unwrap();
//     loop {
//         print!("{}$ ", current_dir.display());
//         stdout().flush().unwrap();
//         stdin().read_line(&mut input).unwrap();
//
//         if input.trim_end() == "exit" {
//             break;
//         }
//
//         let expression = parse_to_expression(&input);
//         execute_expression(expression);
//         input.clear();
//     }
//     // print!("$ ");
//     // let _ = stdout().flush();
//     // stdin()
//     //     .read_line(&mut input)
//     //     .expect("Did not enter a correct string");
//     // let args: Expression = parseToExpression(&input);
//     // executeExpression(&args);
