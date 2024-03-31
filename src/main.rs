use std::process::Command;
use rusqlite::{params, Connection, Result};

fn main() {
    /*
    Command::new("ipconfig")
        .output()
        .expect("Failed to execute command");
    
    we just print out the output of the command as it is since it cant handle the utf-8 issue for some reason
    */

    // Execute the command
    let output = Command::new("ipconfig").output().expect("Failed to execute command");
    
    // Use lossy conversion to handle invalid UTF-8 sequences
    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully:\n{}", output_str);
        
        // Save the output to SQLite
        if let Err(e) = save_output_to_sqlite(&output_str) {
            eprintln!("Failed to save output to SQLite: {}", e);
    }   
    }
    else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Command failed to execute:\n{}", error_str);
    }
}


fn save_output_to_sqlite(output: &str) -> Result<()> {
    // Open a connection to the SQLite database
    // ? at the end of this line means that the function will return an error if it fails
    let conn = Connection::open("command_output.db")?;

    // Create a table to store the command output
    // Note: In a real application, you might want to check if the table already exists

    // Create the table
    // in table , we are gonna have id and output and the id is going to be the primary key
    // text not null means that the output cannot be empty
    // id is going to be sequential and it will be the primary key
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_output (
                  id INTEGER PRIMARY KEY,
                  output TEXT NOT NULL
                  )",
        [],
    )?;

    // Insert the command output into the table
    // put ouptu into the table as a string
    // ? at the end of this line means that the function will return an error if it fails
    conn.execute(
        "INSERT INTO command_output (output) VALUES (?1)",
        params![output],
    )?;

    Ok(())
}