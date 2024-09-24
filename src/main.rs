use std::{io::{stdout, Read, Write}, process::{Command, ExitCode, Stdio}};

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    if let Some(program) = args.next() {
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .unwrap();
        let mut buf = [0u8];
        let mut was_letter = false;
        let mut fuck_stdout = stdout();
        let mut child_stdout = child.stdout.take().unwrap();
        while let Ok(1) = child_stdout.read(&mut buf) {
            was_letter = if buf[0].is_ascii_alphabetic() {
                if !was_letter {
                    fuck_stdout.write(
                        if buf[0].is_ascii_uppercase() {
                            b"Fuck"
                        } else {
                            b"fuck"
                        }
                    ).unwrap();
                }
                true
            } else {
                fuck_stdout.write(&buf).unwrap();
                false
            };
        }
        ExitCode::from(
            child
                .wait()
                .unwrap()
                .code()
                .unwrap_or(0) as u8
        )
    } else {
        println!("Syntax: fuck {{command}}...");
        ExitCode::FAILURE
    }
}
