
 // Stage 1:
 // I want to add bills including name and amount owed.
 // I want to view existing bills.

 //  == Manage Bills ==
 //  1. Add bill
 //  2. View bill
 //  3. Remove bill
 //  4. Update bill
 //  5. Bill total

 //  Enter selection:

use std::io;

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
    TotalBill
}

impl MainMenu {
    // Return MainMenu variants
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            "5" => Some(Self::TotalBill),
            _ => None
        }
    }

    fn show_menu() {
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bill");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total \n");
        println!("Enter selection: \n")
    }
}


// get user input
fn get_input() -> Option<String> {
    let mut buffer = String::new();
   while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your number again.")
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        return None
    };
    Some(input)
}

fn main() {
   MainMenu::show_menu();
   
   
}
