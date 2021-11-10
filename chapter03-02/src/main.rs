fn main() {
    let t = ("masuda", 30);
    println!("name is {} age {}", t.0, t.1);

    let name = "masuda";
    let age = 30;
    let t = (name, age);
    println!("name is {} age {}", t.0, t.1);

    let a = ["春", "夏", "秋", "冬"];
    println!("最初の季節 {}", a[0]);
    println!("最後の季節 {}", a[3]);
}
