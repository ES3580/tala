
//use std::collections::Ve
#[derive(Debug)]


pub enum Mode
{
    STANDARD,
    WHITELIST,
    BLACKLIST
}
impl Default for Mode
{
    fn default() -> Self {Mode::STANDARD}
}


struct Channel
{
    mode: Mode,
    name: String,
    //publisher
    //sender
    addressBook : Vec<i32>
    

}
impl Default for Channel
{
    fn default() -> Channel 
    {
        Channel
        {
            mode: Mode::STANDARD,
            name: String::from("c1"),
            addressBook: Vec::new()
        }
    }
}
impl ToString for Channel
{
     fn to_string(&self) -> String
     {
        return format!("Name:{} Mode:{:?}",self.name, self.mode);
     }
}
impl Channel
{
    fn add(&mut self, ip: i32 )
    {
        self.addressBook.push(ip);
    }
}

fn main() {
    let c = Channel { mode: Mode::STANDARD, name: String::from("c1"), ..Default::default()};
    println!("Hello, world!");
    println!("{}", c.to_string());
}
