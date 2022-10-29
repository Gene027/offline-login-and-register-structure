use std::io::Read;
use std::fs::File;
fn main() {

    'ui: loop {
        let mut log:Vec<String> = Vec::new(); //holds login status log file written from fn login
        let mut database: String = String::new();
        let mut f = File::open("database.txt").unwrap();
        f.read_to_string(&mut database).expect("unable to read file to database");
        let choice: u8 = account_mgr::get_input("1.Register\n2.Log in\n3.Logout\n4.Cancel").parse().expect("input a number");

        if *choice == 1 {

             account_mgr::register(database);
        }
        else if *choice == 2 {
            account_mgr::login(&mut log, &database);
        }
        else if *choice == 3 {
            account_mgr::logout(&mut log);
        }
        else if *choice == 4 {
            println!("Closing Application...");
            break 'ui
        }
        else {
            println!("please choose a valid input");
        }
    }
}