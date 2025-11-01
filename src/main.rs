mod error;
mod message;
mod rectangle;
mod std;

fn main() {
    let b = Box::new(5); // 5 on heap
    println!("boxed: {}", b);
}
