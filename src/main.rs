use  std::collections::HashMap; // Class 261
fn main() {//Class 266

    let mut coffe_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffe_pairings.insert(&drink, &milk); // inserting a reference to the String
    coffe_pairings.insert("flat white", "Almond Milk");
    
    // entry API to insert a value if the key does not exist
    coffe_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("Coffee Pairings: {:?}", coffe_pairings);

    coffe_pairings.entry("Cappuccino").or_insert("Pistachio Milk");
    println!("Coffee Pairings after adding Cappuccino: {:?}", coffe_pairings);

}

/*
//Class 265

    let mut coffe_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffe_pairings.insert(&drink, &milk); // inserting a reference to the String
    coffe_pairings.insert("flat white", "Almond Milk");
    println!("{:?}", coffe_pairings);
    coffe_pairings.insert("Latte", "Pristachio Milk");
    println!("Coffee Pairings: {:?}", coffe_pairings);
*/
/*
//Class 263

    let mut coffe_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffe_pairings.insert(&drink, &milk); // inserting a reference to the String
    coffe_pairings.insert("flat white", "Almond Milk");
    println!("Coffee Pairings: {:?}", coffe_pairings.len());
    println!("{} {}", drink, milk);
*/
/*
 //Class 262
    let data = [("Bobby", 10), ("Alice", 20), ("Charlie", 30)];

    let mut years_at_company: HashMap<&str, i32> = HashMap::from(data);
    println!("Years at company: {:?}", years_at_company);

    let charlie = years_at_company.remove("Charlie");
    println!("Removed Ben: {:?}", charlie);
    println!("{}", charlie.unwrap_or(0)); // unwrap_or() returns a default value if the key is not found
    println!("Years at company after removal: {:?}", years_at_company);

    let charlie = years_at_company.remove("charlie");
    println!("Removed Charlie: {:?}", charlie);
*/
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