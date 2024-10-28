# Rust-Authentication-System
This is a authentication system written in Rust, It can register and authenticate user using mysql database. The code have been compiled to .elf 

## Mysql Database
As mentioned the code will store the data into mysql database, below are the sql query to setup the database and table.
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
The requirements are mysql database and Rust. You can setup rust following the steps in their webstie
```
https://www.rust-lang.org/tools/install
```
