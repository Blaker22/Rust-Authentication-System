use mysql::*;
use mysql::prelude::*;
use std::io::{self, Write};
use sha2::{Sha256, Digest};
use rand::Rng;
use hex; 
use std::process::Command;
use regex::Regex;


fn cls() {
    // Determine the operating system and execute the appropriate command
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the screen");
    }
}

fn register(conn: &mut PooledConn) -> Result<()> {
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    io::stdout().flush().expect("Failed to flush stdout"); 
    io::stdin().read_line(&mut username).expect("Failed to read line");

    let username = username.trim();
    if username.len() > 15 {
        println!("Username '{}' is too long. Maximum length is 15 characters.", username);
        return Ok(()); // Return to the main function
    }

    let exists: Option<u32> = conn.exec_first(
        "SELECT COUNT(*) FROM users WHERE username = ?",
        (username,)
    )?;

    if let Some(count) = exists {
        if count > 0 {
            println!("Username '{}' already exists.", username);
            return Ok(()); // Return to the main function
        }
    }

    print!("Enter password: ");
    io::stdout().flush().expect("Failed to flush stdout"); 
    io::stdin().read_line(&mut password).expect("Failed to read line");

    let password = password.trim();
    if password.len() < 8 {
        println!("Password dont't meet the requirement of 8 characters with letters, numbers, and symbols");
        return Ok(()); // Return to the main function
    }

    let letter_regex = Regex::new(r"[a-zA-Z]").unwrap();
    let number_regex = Regex::new(r"[0-9]").unwrap();
    let symbol_regex = Regex::new(r"[!@#$%^&*(),.?\:{}|<>]").unwrap(); // Customize symbols as needed

    if !letter_regex.is_match(password) || !number_regex.is_match(password) || !symbol_regex.is_match(password) {
        println!("Password must contain at least one letter, one number, and one symbol.");
        return Ok(()); // Return to the main function
    }

    let salt: [u8; 16] = rand::thread_rng().gen();
    let salt_hex = hex::encode(&salt); 

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(&salt);
    let hashed_password = hasher.finalize();
    let hashed_password_hex = hex::encode(hashed_password);

    conn.exec_drop(
        "INSERT INTO users (username, password, salt) VALUES (:username, :password, :salt)",
        params! {
            "username" => username,
            "password" => hashed_password_hex,
            "salt" => salt_hex,
        },
    )?;

    println!("User '{}' registered successfully!", username);

    Ok(())
}

fn check(conn: &mut PooledConn) -> Result<()> {
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    io::stdout().flush().expect("Failed to flush stdout"); 
    io::stdin().read_line(&mut username).expect("Failed to read line");

    print!("Enter password: ");
    io::stdout().flush().expect("Failed to flush stdout"); 
    io::stdin().read_line(&mut password).expect("Failed to read line");

    let username = username.trim();
    let password = password.trim();

    let result: Option<(String, String)> = conn.exec_first(
        "SELECT password, salt FROM users WHERE username = :username",
        params! {
            "username" => username,
        },
    )?;

    if let Some((stored_hashed_password, stored_salt)) = result {
        // Decode the stored salt from hex
        let salt_bytes = hex::decode(&stored_salt).expect("Failed to decode salt from hex");

        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(&salt_bytes);
        let hashed_password = hasher.finalize();
        let hashed_password_hex = hex::encode(hashed_password);

        if hashed_password_hex == stored_hashed_password {
            println!("Correct credentials");
        } else {
            println!("Incorrect credentials");
        }
    } else {
        println!("Username not found");
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let url = "mysql://root:root@localhost/account";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    loop {
        println!("Choose an option:");
        println!("1. Register user account");
        println!("2. Check credentials");
        println!("3. Exit");
        println!("----------------------------------------------------------");

        let mut choice = String::new();

        println!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // Trim newline characters and parse the input
        let choice = choice.trim();

        match choice {
            "1" => {
                register(&mut conn)?;
                println!("Press ENTER to continue");
                let mut input = String::new();
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                cls();
            }
            "2" => {
                check(&mut conn)?;
                println!("Press ENTER to continue");
                let mut input = String::new();
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                cls();
            }
            "3" => {
                println!("Exiting the program...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
                println!("Press ENTER to continue");
                let mut input = String::new();
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                cls();
            }
        }
    }

    Ok(())
}