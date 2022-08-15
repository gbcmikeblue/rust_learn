mod nice;

mod good {
    pub fn get_nice_good_hello() {
        crate::nice::good::hello();
        println!("this is mode good::get_nice_good_hello");
    }
}

fn main() {
    
    // nice::hello();
    // crate::nice::good::hello();
    crate::good::get_nice_good_hello();

    println!("this is main.rs");
}
