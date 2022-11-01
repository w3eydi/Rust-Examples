#[test]
fn printing() {
    println!("ECS Library! It's working.");
    
    let cat_name = Some("Xilbe");
    let something = dbg!(cat_name);
    
    println!("{} passing the test!", something.unwrap());
}