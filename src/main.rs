
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
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
    TotalBill
}

impl MainMenu {
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
}


fn main() {
   
   
}
