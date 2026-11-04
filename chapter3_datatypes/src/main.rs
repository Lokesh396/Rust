fn main() {

    // Scalar Data Types
    // Integer
    let pos_int: u32 = 120;
    let neg_int: i32 = -120;
    println!("{}, {}", pos_int, neg_int);
    //float
    let _flo: f32 = 8.3;

    //char
    let _chr: char = 'a';

    //boolean
    let _is_tracked: bool = false;


    // Compound Types

    //Tuple
    let tup = (1,2,3);
    let new_tup: (i32, bool, char) = (1, true, 'D');
    let index_0 = new_tup.0;
    let index_1 = new_tup.1;
    let index_2 = new_tup.2;
    println!("{index_0}, {index_1}, {index_2}, {:?}", tup);

    //Array
    let _arr = [1, 2, 3, 4, 5];
    let a:[i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("{}, {}, {:?}", a[1], a[3], b);
}
