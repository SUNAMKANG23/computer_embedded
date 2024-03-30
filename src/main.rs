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
    let conn = Connection::open("command_output.db")?;

    // Create a table to store the command output
    // Note: In a real application, you might want to check if the table already exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS command_output (
                  id INTEGER PRIMARY KEY,
                  output TEXT NOT NULL
                  )",
        [],
    )?;

    // Insert the command output into the table
    conn.execute(
        "INSERT INTO command_output (output) VALUES (?1)",
        params![output],
    )?;

    Ok(())
}