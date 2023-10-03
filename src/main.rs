mod inventory;

fn main() {
    inventory::inventory::add_new_item();
    inventory::inventory::remove_item();
    inventory::inventory::display_inventory();
    inventory::inventory::calculate_value();

    let _item = inventory::Item {
        name: String::from("parsnip"),
        price: 5.99,
        quantity: 14,
    };
    println!("I have {} {} at {}$ each.",_item.quantity , _item.name, _item.price);
}
