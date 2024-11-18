#[derive(Debug)]
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String) 
}

fn main() {
    let home=IpAddrKind::V4(127,0,0,1);
    let loopback=IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);
    let op = None;
    let x=2;
    let sum =x+op.unwrap_or(0);
    println!("Sum = {}",sum)
}

fn route(ip: IpAddrKind){
    println!("Routing request to {:?}",ip);
}
