pub fn area_of_circle() {
    use std::io;
    fn main() {
    let mut input = String ::new();
    println!("Input radius : ");
    io::stdin().read_line(& mut input);
    let input :f32 = input.trim().parse().unwrap();
    let area :f32= (input*input)*(3.14);
    println!("Area of circle with radius {} is {}",input,area);
    }  
}