use accessor_macro::Accessor;
use uintx::u24;

#[derive(Accessor, Debug)]
struct Person {
    #[allow(unused)]
    #[accessor(get, set)]
    name: String,
    #[accessor(get, set, range=[0, 200])]
    age: i32,
    #[accessor(get, set, range=[u24::from(0), u24::from(100)])]
    number: u24,
}

fn main() {
    let mut person = Person {
        name: "Alice".to_string(),
        age: 25,
        number: u24::from(50),
    };
    println!("person: {:?}", person);
    println!("--------------------------------------");
    println!("set age to -1:");
    assert!(!person.set_age(-1));
    println!("person: {:?}", person);
    println!("--------------------------------------");
    println!("set age to 18:");
    assert!(person.set_age(18));
    println!("person: {:?}", person);
    println!("--------------------------------------");

    println!("set number to 0:");
    assert!(person.set_number(u24::from(0)));
    println!("person: {:?}", person);
    println!("--------------------------------------");

    println!("set number to 101:");
    assert!(!person.set_number(u24::from(101)));
    println!("person: {:?}", person);
    println!("--------------------------------------");
}
