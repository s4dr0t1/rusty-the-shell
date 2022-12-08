use std::io;
use std::process;

/// The main driving code of this module
/// Does the following:
///     * Runs a command given in by the user
///     * Shows the user prompt
///     * Error handling
///     * Flushing the STDOUT buffer
pub fn run_command() -> Result<(), anyhow::Error> {
    // The user prompt
    print!("> ");

    // Flush STDOUT buffer
    io::stdout().flush()?;

    // Take the user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    // Trim the user input so that it has no white-spaces
    let mut command_identifiers = user_input.trim().split_whitespace();
    // Define program = name of the program we're trying to execute
    let program = command_identifiers.next();

    match program {
        Some(program) => {
            let program_arguments = command_identifiers;
            let mut child = process::Command::new(program)
                .args(program_arguments)
                .spawn()?;
            // Don't accept any other command until the previous one is complete (blocking concept)
            child.wait()?;
            return Ok(());
        },
        None => {
            println!("No input provided");
            return Ok(());
        }
    }
}
