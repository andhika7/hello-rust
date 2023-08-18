fn main() {
    println!("Hello, world!");
    // test menambahkan komen
    println!("{} days",31);

    // print cara berbeda
    println!("{aa}{bb}{cc}", aa="print ", bb="cara ", cc="beda");
    
    let mut _ss: i16 = 80;
    println!("basis 10: {}",_ss);
    println!("basis 2 (biner): {:b}",_ss);
    println!("basis 8 (okta): {:o}",_ss);
    println!("basis 16 (heksa): {:x}",_ss);

    println!("{0}, this is {1}", "hello", "sunday");
    println!("{0} {1}","hello", "world");

    // penambahan '0' pada angka
    println!("{angka:>5}", angka=7);  //    7
    println!("{angka:0>5}", angka=7); //00007
    println!("{angka:0<5}", angka=7); //70000
    println!("{angka:0>lebar$}", angka=7, lebar=5); //00007

    /* ********************* PR unsolved ************************************
    
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    use std::fmt::Display;
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
    */

}
