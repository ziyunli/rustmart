use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

#[derive(Clone, Debug)]
pub struct CartProduct {
    product: Product,
    quantity: i32,
}
