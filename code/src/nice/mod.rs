
pub fn _hello() {
    println!("mod nice:hello");
}

pub mod good {
    pub fn hello() {
        println!("mod nice:good:hello");
    }
}