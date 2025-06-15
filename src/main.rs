use  std::collections::HashMap; // Class 261
fn main() { //Class 262
    let data = [("Bobby", 10), ("Alice", 20), ("Charlie", 30)];

    let mut years_at_company: HashMap<&str, i32> = HashMap::from(data);
    println!("Years at company: {:?}", years_at_company);

    let charlie = years_at_company.remove("Charlie");
    println!("Removed Ben: {:?}", charlie);
    println!("{}", charlie.unwrap_or(0)); // unwrap_or() returns a default value if the key is not found
    println!("Years at company after removal: {:?}", years_at_company);

    let charlie = years_at_company.remove("charlie");
    println!("Removed Charlie: {:?}", charlie);
}
/*
//Class 261
   let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 19.99);
    menu.insert(String::from("Tuna"), 19.99);
    menu.insert(String::from("Burger"), 14.99);

    println!("Menu: {:?}", menu);

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Japan", "Tokyo");

    println!("Country Capitals: {:?}", country_capitals);
 */