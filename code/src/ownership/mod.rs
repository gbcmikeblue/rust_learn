pub mod main {

    #[allow(dead_code)]
    pub fn demo1() -> () {
        println!("this is demo1");
        let str1 = String::from("你好");
        println!("str1 : {}", str1);
    }

    #[allow(dead_code)]
    pub fn demo2() -> () {
        println!("this is demo2");
        let x = "hello x!";
        let y = x;
        println!("x : {}, y: {}", x, y);
    }

    #[allow(dead_code)]
    pub fn demo3(a: i32) -> () {
        let x = 5;
        let y = &x;
        assert_eq!(x, a);
        assert_eq!(*y, a); // 解引用 y
    }

    #[allow(dead_code)]
    pub fn demo4() -> () {
        let s = String::from("Hello");
        // 不可变引用
        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        let len = calculate_length(&s);
        println!("s len : {}", len);
    }

    #[allow(dead_code)]
    pub fn demo5() -> () {
        // 可变引用
        // 想在函数内部 修改 变量 的 值
        let mut s = String::from("Hello");

        fn change(s: &mut String) {
            s.push_str(", World!");
        }
        change(&mut s);
        println!("s : {}", s);
        // 可变引用与不可变引用 同时存在
        // rust 现在已 优化 为 NLL
    }

    #[allow(dead_code)]
    pub fn demo6() -> () {
        // 悬垂引用

        let reference_noting = dangle();

        fn dangle() -> String {
            let s = String::from("hello");
            // &s // 执行返回之后，s变量离开函数作用域 ，内存被回收
            // 所以 返回的引用 为无效 的 String引用
            // 解决办法就是直接返回 String 转移变量所有权到外部
            s
        }

        println!("reference_noting return : {}", reference_noting);
        
    }

}