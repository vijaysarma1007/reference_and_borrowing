fn main() {
    //reference and borrowing
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    let mut car = String::from("Red");
    let ref1 = &mut car;
    ref1.push_str(" and silver");
    let ref2 = &car;

    println!("{ref2}");

    let coffee = String::from("Mocha");
    let a = &coffee;
    let b = a;

    println!("{a} and {b}");
 

    //Array of booleans
    let regsitrations = [true, false, true];
    let first = regsitrations[0];
    println!("{first} and {regsitrations:?}");

    //Array of heap string
    let languages = (String::from("Rust"), String::from("JavaScript"));
    let first = &languages.0;
}

fn create_city() -> String {
    String::from("new york")
}

fn add_flour(meal: &mut String) {
    meal.push_str(" Add flour");
}

fn show_my_meal(meal: &String) {
    println!("meal steps: {meal}");
}
