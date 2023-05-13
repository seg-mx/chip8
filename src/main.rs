use std::{env, io::stdout};

use chip8::{config::Config, emulator::Emulator, quit, run, setup};

fn main() {
    let mut stdout = stdout();

    if let Err(err) = setup(&mut stdout) {
        quit(
            &mut stdout,
            format_args!("Error while setting up terminal: {err}"),
        );
    }

    let mut args = env::args();

    // Skip program path
    args.next();

    let config = match Config::new(args) {
        Ok(config) => config,
        Err(err) => quit(
            &mut stdout,
            format_args!("Problem parsing arguments: {err}"),
        ),
    };

    let emulator = Emulator::new(config, &mut stdout);

    if let Err(err) = run(emulator, &mut stdout) {
        eprintln!("Application error: {err}");
    }
}
