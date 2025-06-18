use std::collections::HashMap;

fn main() { 
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot Dog", "Burgers", "Pretzels"]);

    println!("{:?}", sauces_to_meals.remove("Mayonnaise").unwrap()); // Remove Mayonnaise and print its value

    let mustard_meals = sauces_to_meals.get("Mustard");
    match mustard_meals {
        Some(meals) => println!("The meals were {:?}", meals),
        None => println!("There were no meals for that sauce! Oh no!"),
    }

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings" ]);

    println!("{:?}", sauces_to_meals);

}


/*
use  std::collections::HashSet;// Class 267

//Class 268
    let mut concert_queue: HashSet<&str> = HashSet::new(); // Boris, melissa
    let mut movie_queue: HashSet<&str> = HashSet::new(); // Boris, Phil

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue)); // Union of concert and movie queues
    println!("{:?}", movie_queue.union(&concert_queue)); // Union of movie and concert queues

    println!("{:?}", concert_queue.difference(&movie_queue)); // Difference of concert and movie queues
    println!("{:?}", movie_queue.difference(&concert_queue)); // Difference of movie and concert queues

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue)); // Symmetric difference of concert and movie queues
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue)); // Symmetric difference of movie and concert queues

    println!("{:?}", concert_queue.is_disjoint(&movie_queue)); // Check if concert and movie queues are disjoint
    println!("{:?}", movie_queue.is_disjoint(&concert_queue)); // Check if movie and concert queues are disjoint

    let mut attendees: HashSet<&str> = HashSet::new();
    attendees.insert("Boris");
    println!("{:?}", attendees.is_subset(&concert_queue)); // Check if attendees is a subset of concert queue

    println!("{:?}", concert_queue.is_superset(&attendees)); // Check if concert queue is a superset of attendees

*/

/*
//Class 267
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("Concert Queue: {:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("Concert Queue after adding Molly and Megan: {:?}", concert_queue);
    println!("{}", concert_queue.len());

    concert_queue.insert("Molly"); // Duplicate entry, will not be added
    println!("{:?}", concert_queue);

    println!("{}", concert_queue.remove("Megan"));// Remove Megan from the queue
    println!("{}", concert_queue.remove("Franny"));// Remove Franny from the queue
    println!("Concert Queue after removals: {:?}", concert_queue);

    println!("{}", concert_queue.contains("Molly")); // Check if Molly is in the queue
    println!("{}", concert_queue.contains("Fred")); // Check if Fred is in the queue

    println!("{:?}", concert_queue.get("Molly")); // Get a reference to Molly in the queue
    println!("{:?}", concert_queue.get("Joe")); // Get a reference to Joe in the queue

*/
 /*
 //Class 266

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
*/
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