use std::io;
use std::fs::OpenOptions;
use std::io::Write;

pub fn register(database: String) {
    let username = get_input("enter new username");
    let password =  get_input("enter new password");
    
    //check if username is already in database
    if !used_username(&username, &database) == true {
        panic!("username already exists");
    }

    if !valid_password(&password) {
        panic! ("password too short");
    }
    
    let user = format!("\n{} {}", username.clone(), password); // \n for line break when writing

    let mut f = OpenOptions::new()
    .write(true).append(true).read(true).open("database.txt").unwrap();

     f.write_all(user.as_bytes()).expect("writing to registry failed");
        println!("you successfully registered as {}", username);
       }
    
pub fn login (log:&mut Vec<String>, database: &String) {
    let username = get_input("enter your username");
    let password =  get_input("enter your password");
    //check for login status
    let user = get_user(&username, &database);
    
    if user.is_empty() {
        println! ("sorry, no registered user");
    } else{
        let invalid = verify(&username, password, user, log);
        if invalid == false {
            let usr = username.clone();
            log.push(usr.to_string());
    
            println!("successfully logged in as {}", username);
        } else{
            println!("false login");
        }

    }
    
}

pub fn logout(log:&mut Vec<String>) {
    let username = get_input("enter your username to log out");
    let _log = log.clone();
    for un in _log.iter(){
        if *un == username.as_str(){
            let index = _log.iter().position(|x| *x == username.as_str())
            .unwrap();
            log.remove(index.clone());
            println!("successfully logged out");
        }
        else {
            println!("false logout: user is not logged");
        }
    }
}

pub fn verify(username: &String, password: String, user: String, log:&Vec<String>) -> bool{
    let mut check = String::new();
    if not_logged(&username, log) == true {
    let mut _user: Vec<_> = user.split(" ").map(|string| string.trim()).collect();
    if password == _user[1] {
        check.push_str(username);
    }
     else{ 
        println!("incorrect password")
    }
}
else{
    println!("already logged in")
}
   
    check.is_empty()
}

pub fn not_logged (username: &String, log:&Vec<String>) -> bool {
    let mut user = String::new();
    for i in log.iter(){
        if *i == username.to_string() {
            println!("sorry already logged in");
            user.push_str(username.as_str())
        } else{
            continue;
        }
    }
    user.is_empty()
}


pub fn get_input(prompt:&str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok (_) => {},
        Err(_) => panic! ("failed to read line"),
    }
    input.trim().to_string()
}


pub fn used_username(username: &String, database: &String) -> bool {
    let mut check = String::new();
    for line in database.lines(){
        let user: Vec<_> =line.split(" ").map(|x| x.trim()).collect();
        if user[0] == username.as_str(){
           check.push_str(&username.as_str())
        } else{
           continue;
        }
    }
    check.is_empty()
}

pub fn valid_password(password: &String) -> bool {
    password.len() >= 5
}

pub fn get_user(username: &String, database: &String) -> String {
    let mut found_user = String::new();

    for line in database.lines(){
        if line.contains(username.as_str()){
            found_user.push_str(line);
        } else{
            continue;
        }
    }
    found_user
}
