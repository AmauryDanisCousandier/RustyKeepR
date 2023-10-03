struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

fn main() {
    let _item = Item {
        name: String::from("parsnip"),
        price: 5.99,
        quantity: 14,
    };
    println!("I have {} {} at {}$ each.",_item.quantity , _item.name, _item.price);
}
