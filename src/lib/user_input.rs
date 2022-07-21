/*
 * The main driving code of this module
 * Does the following:
     * Runs a command given in by the user
     * Shows the user prompt
     * Error handling
     * Flushing the STDOUT buffer
 */
pub fn run_command()
{
    loop {
        //The user prompt
        println!("> ");

        //Flush STDOUT buffer
        use std::io::Write;
        std::io::stdout().flush();

        //Run the command
        match private::run_command()
        {
            Ok(_) => continue,
            Err(some_error) => println!("An error occurred: {}", some_error),
        }
    }
}



//This shit is private to other parts, defined in a private module for better readability
mod private 
{
    //External libraries and modules
    use std::
    {
        io,
        process
    };


    ///Run the command based on the user input
    pub fn run_command() -> Result<(), anyhow::Error>
    {
        //Take the user input
        let user_input = take_user_input()?;

        //Trim the user input so that it has no white-spaces
        let mut command_identifiers = user_input.trim().split_whitespace();
        //Define program = name of the program we're trying to execute
        let program = command_identifiers.next();

        match program {
            Some(program) => {
                let program_arguments = command_identifiers;
                let mut child = process::Command::new(program)
                    .args(program_arguments)
                    .spawn()?;
                    //.unwrap(); //you need to handle this later, just keep it as as it now
                //Don't accept any other command until the previous one is complete (blocking concept)
                child.wait()?;
                return Ok(());
            },
            None => 
            {
                println!("No input provided");
                return Ok(());
            }

        }
    }

    ///Take in user input from the user and return an error otherwise
    fn take_user_input() -> Result<String, io::Error>
    {
        //Variable used to store user input
        let mut user_input = String::new();

        //Take input from the user
        io::stdin().read_line(&mut user_input)?;

        //Return the user input (in case of any error, the ? operator will deal with it)
        Ok(user_input)
    }
}
