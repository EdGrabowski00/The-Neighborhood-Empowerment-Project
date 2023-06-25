use std::{
    collections::HashMap,
    io::{self, Read},
    thread,
    sync::{Arc, Mutex},
};

fn main() {
    // Create a map of community members
    let mut members = HashMap::new();

    // Create a thread-safe reference counter
    let members_count = Arc::new(Mutex::new(0));

    loop {
        // Ask for the user's input
        println!("Please enter your name or 'exit' to quit:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if name == "exit" {
            break;
        }

        // Increment the counter and print the current count
        let mut members_count = members_count.lock().unwrap();
        *members_count += 1;
        println!("You are member #{}!", members_count);

        // Create a thread for the current user
        let members_clone = members.clone();
        let thread_name = format!("user-{}", name);
        thread::spawn(move || {
            // Add the current user to the list of members
            members_clone.insert(name, *members_count);

            // Print a message to the console
            println!("Welcome, {}, to the community!", name);
        }).name(thread_name).unwrap();
    }

    println!("The community has {} members.", members.len());
    println!("Thanks for being a part of it!");
}