
pub fn _hello() {
    println!("mod nice:hello");
}

pub mod good {

    #[allow(dead_code)]
    pub fn hello() {

        let i: i32 = -10;

        println!("mod nice:good:hello, i is {}", i);

        // println!();

        let nums = [1,2,3,4,5];
        let ref_nums = &nums;
        
        for num in ref_nums {
            println!("{}", num);
        }
    }
}