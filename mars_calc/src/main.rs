use std::io;

fn main() {
    println!("Enter your weight: \n");
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight: f32 = calculate_weight_on_mars(weight);

    println!("Your weight on mars is {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}
