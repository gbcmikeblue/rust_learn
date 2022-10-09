#![allow(dead_code)]
pub mod main {

    pub fn demo1() -> () {
        // struct and enum

        // slice
        let s = String::from("hello world");

        // hello
        let hello = &s[0..5];
        // let hello = &s[..5];

        let world = &s[6..11];
        println!("{} {}", hello, world);

    }

    pub fn demo2() -> () {
        // struct and enum

        let s = String::from("hello world");


        fn first_word(s: &String) -> &str {
            &s[..1]
        }
        
        // s.clear();

        let word = first_word(&s);

        println!("word : {}", word);

    }
    
    pub fn demo3() {

        let string1 = String::from("hello world");
        say(&string1);
        say(&string1[..]);

        say(string1.as_str());

        fn say(s: &str) {
            println!("s : {}", s);
        }
    }

    pub fn demo4() {
        let mut s = String::from("hello ");

        s.push('r');// char 只允许长度为1
        s.push('u');
        println!("s : {}", s);
        
        s.push_str("st!");
        println!("s : {}", s);
    }

    pub fn demo5() {
        
        // insert 
        let mut s = String::from("hello,rust!");
        println!("s is {}", s);
        s.insert(6, 'i');

        s.insert_str(7, " love ");

        println!("s is {}", s);
    }

}