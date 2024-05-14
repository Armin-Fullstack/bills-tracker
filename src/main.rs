
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
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bill");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total \n");
        println!("Enter selection: \n")
    }
}


#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64
}
struct ManageBills {
    bills: Vec<Bill>
}


impl ManageBills {
    // created a new function to generate a ManageBills structure
    fn new() -> Self {
        Self {
            bills: vec![]
        }
    }

    // push bill to the bills vector
    fn add(&mut self , bill: Bill) {
        self.bills.push(bill)
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
         None
    } else {
        Some(input)
    }
}

// for converting string to f64, created a function
fn get_bill_amount() -> Option<f64> {
    println!("Enter amount:");
   loop {
    let input = match get_input() {
        Some(input) => input,
        None => return None
    };

    if &input == "" {
        return None
    }

    let parsed_input: Result<f64, _> = input.parse();
    match parsed_input {
        Ok(amount) => return Some(amount),
        Err(_) => println!("Please enter a number.")
    }

   }
    
}


mod menu {
    use crate::{get_bill_amount, get_input, Bill, ManageBills};

    // get name and amount from user and then add them to bill 
   pub fn add_bill(bills: &mut ManageBills) {
        println!("Enter bill name:");
        // get bill name
        let name = match get_input() {
            Some(input) => input,
            None => return
        };

        // get amount
        let amount = match get_bill_amount() {
            Some(input) => input,
            None => return
        };

        // created a bill
        let bill = Bill {name , amount};
        
        // added bill
        bills.add(bill);
        println!("Bill added successfully.")
    }

    // print all bills
    pub fn view_bills(bills: &ManageBills) {
        if bills.bills.is_empty() {
            println!("There aren't any bills.")
        }
        for bill in &bills.bills {
            println!("{:?}" , bill)
        }
    }
}


fn main() {
    use menu::{add_bill , view_bills};

    let mut bills = ManageBills::new();

  loop {
      MainMenu::show_menu();
       let input = get_input().expect("No data entered");
        match MainMenu::from_str(&input) {
           Some(MainMenu::AddBill) => add_bill(&mut bills),
           Some(MainMenu::ViewBill) => view_bills(&bills),
           _ => ()
        }
  } 
}
