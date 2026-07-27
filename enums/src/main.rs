enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
   }

/*
*   Rust does not use null values but implements an Option enum:
*
*   pub enum Option<T> {
*       None,
*       Some(T),
*   }
*
*/