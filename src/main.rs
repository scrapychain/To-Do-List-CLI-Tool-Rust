use std::io;
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
                println!("Enter index that you want to delete (0 to n-1)");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                let index_number: usize = index.trim().parse().expect("Please enter a valid index");
                items.remove(index_number);

                println!("Removed the item at index {}", index);
            }
            "edit" => {
                println!("Enter index  of the item that you want to edit:");
                let mut index = String::new();
                let mut updated_item = String::new();
                io::stdin().read_line(&mut index).unwrap();
                println!("Enter new item: ");
                io::stdin().read_line(&mut updated_item).unwrap();

                let index_num: usize = index.trim().parse().expect("Enter a valid index.");
                items[index_num] = updated_item;
                println!("Item updated at index {}", index_num);
            }
            "list" => {
                if items.len() == 0 {
                    println!("Your List is Empty.");
                } else {
                    for (i, item) in items.iter().enumerate() {
                        println!("{} : {}", i, item);
                    }
                }
                println!("------------------------------------");
                println!("Total item in the list: {}", items.len());
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
