pub fn run_command()
{
    loop {
        //The user prompt
        use std::io::Write;
        println!("> ");

        //Flush STDOUT buffer
        std::io::stdout().flush();
        private::run_command();
    }
}



mod private {
    //External libraries and modules
    use std::
    {
        io,
        process
    };

    ///Run the command based on the user input
    pub fn run_command()
    {
        let user_input = take_user_input();
        match user_input {
            
            //If we get a string from user input, call the command
            Ok(input) => {
                //Trim the user input so that it has no white-spaces
                let command = input.trim();
                let mut child = process::Command::new(command)
                    .spawn()
                    .unwrap(); //you need to handle this later, just keep it as as it now
                //Don't accept any other command until the previous one is complete (blocking concept)
                child.wait();
            },
            //Otherwise handle the error
            Err(error) => println!("An error occurred: {}", error)
        }
    }


    ///Take in user input from the user and return an error otherwise
    fn take_user_input() -> Result<String, io::Error>
    {
        //Define the variable which will be used to hold user input
        let mut user_input = String::new();

        //Take input from the user
        io::stdin().read_line(&mut user_input)?;

        Ok(user_input)
    }
}

