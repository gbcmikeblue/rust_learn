
mod good {

    #[allow(dead_code)]
    pub fn get_nice_good_hello() {
        crate::nice::good::hello();
        println!("this is mode good::get_nice_good_hello");
    }
}
// 引入 nice mod
mod nice;

// 引入 func mod
// mod func;

// mod ownership;

mod complex_types;

fn main() {
    
    // nice::hello();
    // crate::nice::good::hello();
    // crate::good::get_nice_good_hello();

    // let define_number: i32 = 100;
    // let add_five_res = crate::func::main::add_five(define_number);
    // println!("add_five_res res is {}", add_five_res);

    // crate::ownership::main::demo3(5);
    // crate::ownership::main::demo4();
    // crate::ownership::main::demo5();
    // crate::ownership::main::demo6();

    // crate::complex_types::main::demo1();
    // crate::complex_types::main::demo2();
    // crate::complex_types::main::demo3();

    // crate::complex_types::main::demo4();
    // crate::complex_types::main::demo5();
    // crate::complex_types::main::demo6_replace();
    // crate::complex_types::main::demo7_delete();
    crate::complex_types::main::demo8_concatenate();

    println!("this is main.rs");
}
