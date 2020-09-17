

mod channel;

fn main() {
    let c = channel::Channel { mode: channel::Hype::BLACKLIST, /*name: String::from("c1"),*/ ..Default::default()};
    println!("Hello, world!");
    println!("{}", c.to_string());
}
