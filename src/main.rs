
 // Stage 1:
 // I want to add bills including name and amount owed.
 // I want to view existing bills.

 // Stage 2:
 // I want to remove bills

 // Stage 3:
 // I want to edit existing bills.
 // I want to go back if I change my mind.

 //  == Manage Bills ==
 //  1. Add bill
 //  2. View bill
 //  3. Remove bill
 //  4. Update bill
 //  5. Bill total

 //  Enter selection:

use std::{collections::HashMap, io};


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
    bills: HashMap<String , Bill>
}


impl ManageBills {
    // created a new function to generate a ManageBills structure
    fn new() -> Self {
        Self {
            bills: HashMap::new()
        }
    }

    // push bill to the bills vector
    fn add(&mut self , bill: Bill) {
        self.bills.insert(bill.name.to_owned(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.bills.values().collect()
    }
    // remove bill
    fn remove(&mut self , name: &str) -> bool {
        self.bills.remove(name).is_some()
    }

    // update bill 
    fn update(&mut self , bill_name: &str, name: &str , amount: f64) -> bool {
        match self.bills.get_mut(bill_name) {
            Some(bill) => {
                bill.name = name.to_owned();
                bill.amount = amount;
                true
            },
            None => false

        }
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
fn get_bill_amount(text: &str) -> Option<f64> {
    println!("{}" , text);
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

// check if there are any bills
fn is_bills_empty(bills: &ManageBills) -> bool {
    if bills.get_all().is_empty() {
        println!("There aren't any bills.");
        return true;
    }
    false
}

mod menu {
    use crate::{get_bill_amount, get_input, is_bills_empty ,  Bill, ManageBills};

    // get name and amount from user and then add them to bill 
   pub fn add_bill(bills: &mut ManageBills) {
        println!("Enter bill name:");
        // get bill name
        let name = match get_input() {
            Some(input) => input,
            None => return
        };

        // get amount
        let amount = match get_bill_amount("Enter amount:") {
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
        if is_bills_empty(bills) {
            return;
        }
        for bill in bills.get_all() {
            println!("{:?}" , bill)
        }

        println!("Hi")
    }

    // remove a bill base on its name
   pub fn remove_bill(bills: &mut ManageBills) {
    
    if is_bills_empty(bills) {
        return;
    }

    for bill in bills.get_all() {
        println!("{:?}" , bill)
    }
    println!("Enter bill name to remove:");

    let name = match get_input() {
        Some(input) => input,
        None => return
    };

    if bills.remove(&name) {
        println!("The bill was removed successfully.")
    } else {
        println!("The bill not found.")
    }

    }

    pub fn update_bill(bills: &mut ManageBills) {
       if is_bills_empty(bills) {
        return;
       }

        for bill in bills.get_all() {
            println!("{:?}" , bill)
        }

        println!("Enter bill name:");
        let bill_name = match get_input() {
            Some(name) => name,
            None => return
        };
        println!("Enter new name:");
        let new_bill_name = match get_input() {
            Some(name) => name,
            None => return
        };

        let new_bill_amount = match get_bill_amount("Enter new amount:") {
            Some(amount) => amount,
            None => return
        };

        if bills.update(&bill_name , &new_bill_name, new_bill_amount) {
            println!("The bill was updated successfully");
        } else {
            println!("The bill not found.")
        }

    }

    // calculate total bills
    pub fn total_bill(bills: &ManageBills) {
        if is_bills_empty(bills) {
            return;
        }

        let mut total_bills = 0.0;
        for bill in bills.get_all() {
            total_bills  += bill.amount
        }
        println!("total bills: {:?}" , total_bills)

    }


}


fn main() {
    use menu::{add_bill , view_bills , remove_bill, update_bill, total_bill};

    let mut bills = ManageBills::new();

  loop {
      MainMenu::show_menu();
       let input = get_input().expect("No data entered");
        match MainMenu::from_str(&input) {
           Some(MainMenu::AddBill) => add_bill(&mut bills),
           Some(MainMenu::ViewBill) => view_bills(&bills),
           Some(MainMenu::RemoveBill) => remove_bill(&mut bills),
           Some(MainMenu::UpdateBill) => update_bill(&mut bills),
           Some(MainMenu::TotalBill) => total_bill(&bills),
           _ => ()
        }
  } 


}
