fn main() {
    let mut arr=[10,20,30];
    update(&mut arr);
    println!("inside main {:?}",arr);
}

fn update(arr:&mut[i32;3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("inside update {:?}",arr);
}
