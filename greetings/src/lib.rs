use hs_bindgen::*;


#[hs_bindgen]
fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[hs_bindgen]
fn hello_rust(str: &str) -> String {
    format!("Hello {str}, this is Rust speaking")
}
