use std::io::{self, Read};
fn main() {
    let mut items: Vec<String> = Vec::new();

    loop {
        println!("Todo CLI - Enter command (add/list/remove/clear/quit):");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "add" => {
                println!("Enter an item");
                let mut item = String::new();
                io::stdin().read_line(&mut item).unwrap();
                items.push(item);
                println!("Added!");
            }
            "remove" => {
                println!("Enter item number that you want to delete (0 to n-1)");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index_number: usize =
                    index.trim().parse().expect("Please enter a valid number");
                items.remove(index_number);

                println!("Removed the item on index {}", index);
            }
            "list" => {
                if items.len() == 0 {
                    println!("Your List is Empty.");
                } else {
                    for (i, item) in items.iter().enumerate() {
                        println!("{} : {}", i, item);
                    }
                }
            }
            "clear" => {
                items.clear();
                println!("Cleared!");
            }
            "quit" => break,
            _ => println!("Unknown Command"),
        }
    }
}
