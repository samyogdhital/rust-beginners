pub fn run() {
    //Print to console
    println!("Hello from print.rs file");

    //Basic Formatting
    println!("{} {}", "Samyog", "Dhital",);

    //Posititonal Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Samyog", "Dang", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to do {activity}",
        name = "Samyog",
        activity = "Business"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    //Bacic math
    println!("10+10={}", 10 + 10);
}
