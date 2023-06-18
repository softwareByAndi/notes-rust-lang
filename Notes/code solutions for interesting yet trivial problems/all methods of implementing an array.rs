#[allow(unused_variables)]
#[allow(dead_code)]

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("{} : {:?}", slice.len(), slice);
}

fn main() {

    // Fixed-size array (type signature is superfluous)
    let _a1: [i32; 9] =  [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let _a2 =            [1_i32, 2, 3, 4, 5, 6, 7, 8, 9];
    let _a3 =            [0_i32; 9];
    
    let mut _a4 = _a3.clone();
    for i in 0_i32 .. 9 {
        _a4[i as usize] = i;
    }

    let mut _a5 = vec![0; 9];
    _a5[4..].copy_from_slice(&_a4[4..]);
    //_a5[4..].clone_from_slice(&_a4[4..]);
    
    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&_a4[1 .. 4]);
    analyze_slice(&_a4[1 ..= 4]);

    println!("{:?}", _a5);
}

    // OUTPUT

    // borrow a section of the array as a slice
    // 3 : [1, 2, 3]
    // 4 : [1, 2, 3, 4]
    // [0, 0, 0, 0, 4, 5, 6, 7, 8]