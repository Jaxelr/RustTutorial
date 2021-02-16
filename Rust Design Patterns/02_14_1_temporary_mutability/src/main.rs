fn main() {

    //nested block
    let data = {
        let mut data = get_vec();
        data.sort();

        data
    };

    //Data here is immutable
    for i in data {
        println!("data number {}", i);
    }

    //Alternatively you can assign data 
    let mut data2 = get_vec();
    data2.sort();

    let data2 = data2;

    //Data2 here is immutable
    for i in data2 {
        println!("data2 number {}", i);
    }
}


fn get_vec() -> Vec<u64> {
    let mut ids = vec![];

    ids.push(1);
    ids.push(5);
    ids.push(4);

    ids
}