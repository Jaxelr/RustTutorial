enum IpAddressKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    let _four = IpAddressKind::V4(127,0,0,1);
    let _six = IpAddressKind::V6(String::from("::1"));
    
    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}
