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

    pub fn demo6_replace() {
        
        // 1 replace 替换 匹配到的字符串为 新的字符串 并返回一个新的字符串
        let string_replace = String::from("I like rust. Learning rust is my favorite!");
        let new_string_replace = string_replace.replace("rust", "RUST");
        dbg!(new_string_replace); // dbg! 打印变量的宏  还可以的

        // 2 replacen  替换 匹配到的字符串为 新的字符串 
        // 对于有多个匹配结果的情况 还可以设置匹配替换次数 并返回一个新的字符串
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replacen = string_replace.replacen("rust", "RUST", 2);
        dbg!(new_string_replacen);

        // 3 replace_raange 替换字符串中指定范围的字符 操作原字符串  不支持 &str 类型
        // range 解释  x..y  包含 x至y 的索引字符串 但不包含 y这个索引的字符串
        let mut string_replace_range = String::from("I like rust!");
        string_replace_range.replace_range(7..8, "A");
        dbg!(string_replace_range);

    }

    pub fn demo7_delete() -> () {
        // 4 个方法 pop remove trancate clear 操作 删除 字符串
        // 1. pop  从字符串尾巴 取出一个字符串 支持 utf8 中文 3个字节取出
        // 删除并返回字符串的最后一个字符
        let mut string_pop = String::from("rust pop 中文!");
        let p1 = string_pop.pop();
        let p2 = string_pop.pop();
        dbg!(p1);
        dbg!(p2);
        dbg!(string_pop);

        // 2. remove 删除并返回字符串中指定位置的字符
        let mut string_remove = String::from("测试remove方法");
        println!(
            "string_remove 占 {} 个字节",
            std::mem::size_of_val(string_remove.as_str())
        );
        string_remove.remove(0);
        string_remove.remove(3);// 中文占三个字节导致不能准确
        dbg!(string_remove);

        // 3. trancate 删除字符串中从指定位置开始到结尾的全部字符
        let mut string_trancate = String::from("测试trancate方法");
        string_trancate.truncate(6);
        dbg!(string_trancate);

        // 4. clear 清空字符串
        let mut string_clear = String::from("测试clear方法");
        string_clear.clear();
        dbg!(string_clear);


    }

    pub fn demo8_concatenate() {
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        // 使用 &string_rust 会自动解引用为 &str
        let result = string_append + &string_rust;
        let mut result = result + "!";
        result += "!!!";

        dbg!(result);

        let s1 = String::from("hello,");
        let s2 = String::from("rust!");
        // 使用 &string_rust 会自动解引用为 &str
        let s3 = s1 + &s2;
        assert_eq!(s3, "hello,rust!");
        // dbg!(s1);
        dbg!(s2);
    }   
}