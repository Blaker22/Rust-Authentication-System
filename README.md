# Rust-Authentication-System
This authentication system is written in Rust, It can register and authenticate the user using MySQL database. The code has been compiled to .elf 

When registering, the username must be a maximum of 15 characters long and the password needs to be a minimum of 8 characters with a letter, number, and symbol inside.

## MySQL Database
As mentioned the code will store the data into MySQL database, below is the SQL query to set up the database and table.
```
CREATE DATABASE account;
USE account;
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL	
);
```

## Requirements
The requirements are MySQL database and Rust. You can set Rust following the steps on their website.
```
https://www.rust-lang.org/tools/install
```
