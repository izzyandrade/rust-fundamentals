//vector are basically arrays that can grow.

fn main() {
    let mut new_vec: Vec<u128> = vec![1, 2, 3, 4, 5];

    new_vec.push(50);
    new_vec.remove(0);

    for n in new_vec.iter() {
        println!("{}", n);
    }
}