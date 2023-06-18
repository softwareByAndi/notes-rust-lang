fn main() {
    let u: u16 = 300;
    let i: i16 = -128;
    let i2: i16 = -254; // (i8::MAX as i16 * -1) - 2;
    
    // conversions drop the most significant bits until the number fits.
    // basically the same as the % operator
    println!("{:4}_u16 -> {}_u8", u, u as u8);
    println!("\t{:016b} -> {:08b}\n", u, u as u8);
    
    println!("{:4}_i16 -> {}_u8", i, i as u8);
    println!("\t{:016b} -> {:08b}\n", i, i as u8);
    
    // shift is calculated using unsigned version not signed
    println!("{:4}_i16 -> {}_i8 // notice shift is +256, not +{}", i2, i2 as i8, i8::MAX);
    println!("\t{:016b} -> {:08b}\n", i2, i2 as i8);
}

    // OUTPUT

    // 300_u16 -> 44_u8
    //     0000000100101100 -> 00101100
    //
    // -128_i16 -> 128_u8
    //     1111111110000000 -> 10000000
    //
    // -254_i16 -> 2_i8 // notice shift is +256, not +127
    //     1111111100000010 -> 00000010
