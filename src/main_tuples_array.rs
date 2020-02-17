use std::mem;

fn main() {
    let t = (1, 'a', false);
    let f = (2, (1, 'a', false));
    println!("{} {} {}", t.0, t.1, t.2);
    println!("{:?}", f);

    let long_t = ( 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 );
    //println!("{:?}", long_t); -> this cannot be parsed because of length
    let long_arr = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 ];
    println!("{:?}", long_arr);
    
    let xs: [i32; 5] = [4, 5, 6, 7, 8];
    println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));

    let ys = &xs[2..4];
    println!("{} {}", ys[0], ys[1]);

    println!("{:?} {:?}", ys, xs)
}

