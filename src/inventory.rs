pub struct Item {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

pub mod inventory {
    pub fn add_new_item() {
        println!("adds a new item");
    }
    pub fn remove_item() {
        println!("removes an item");
    }
    pub fn display_inventory() {
        println!("displays the inventory");
    }
    pub fn calculate_value() {
        println!("displays the total value of the inventory");
    }
}