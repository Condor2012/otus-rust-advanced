use borrowing_references_slices::storage::Storage;

fn main() {
    let (x, y) = (10, 20);
    let ref_x = {
        let mut storage = Storage::<i32>::new();
        storage.push(&x);
        storage.push(&y);
        match storage.get(0) {
            Some(ref_x) => ref_x,
            None => unreachable!(),
        }
    };

    println!("{ref_x}");
}
