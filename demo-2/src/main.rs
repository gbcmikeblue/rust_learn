fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";

    let chinese = "你好";

    let english = "hello";

    let regions = [southern_germany, chinese, english];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
