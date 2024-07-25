mod my_module;

// 使用模块中的函数
use my_module::utils::greet;

fn main() {
    type_test(1, 2);
    branch_test();
    loop_test();
    borrow_test();
    struct_test();
    enum_test();
    match_test();

    let coin = Coin::Dime;
    let coin_value = value_in_cents(coin);
    println!("coinValue={}", coin_value);

    iterator_test();

    greet("world");

    let result = my_module::utils::add(1, 2);
    println!("result={}", result);

    let result2 = my_module::utils::subtract(1, 2);
    println!("result2={}", result2);
}

// 类型
fn type_test(a: i32, b: i32) -> i32 {
    // int型
    let mut x: i32 = 66;
    x += 1;
    println!("x = {}", x);

    // float
    let mut y: f64 = 3.14;
    y += 1.0;
    println!("y = {}", y);

    // boolean
    let mut z: bool = true;
    println!("z = {}", z);
    z = false;
    println!("z = {}", z);

    // char
    let c: char = 'a';
    println!("c={}", c);

    // str
    let d: &str = "hello world";
    println!("d={}", d);

    return a + b;
}

// 分支
fn branch_test() {
    let score: i32 = 80;
    if score >= 90 {
        println!("优秀");
    } else if score >= 60 {
        println!("良好");
    } else {
        println!("不及格");
    }
}

// 遍历测试
fn loop_test() {
    // while
    let mut count: i32 = 0;
    while count < 5 {
        if count == 3 {
            break;
        }
        println!("count={}", count);
        count += 1;
    }

    // for
    for i in 0..5 {
        if i == 3 {
            break;
        }
        println!("i={}", i);
    }

    // loop
    let mut num = 0;
    loop {
        if num == 3 {
            break;
        }
        num += 1;
        println!("num={}", num);
    }

    let a = [1, 2, 3, 4];
    let b = ["a", "b", "c"];
    for i in a.iter() {
        println!("a={}", i);
    }
    for i in b.iter() {
        println!("b={}", i);
    }
}

// 借用与引用
fn borrow_test() {
    // 普通值是复制，a，b不干扰
    let mut a: i32 = 10;
    a += 1;
    let b = a;
    a += 1;
    println!("a={}, b={}", a, b);

    // 非普通值引用，借用？
    let mut str1 = String::from("hello");
    str1 += " world";
    println!("str1={}", str1);

    // 深拷贝
    let mut str3 = str1.clone();

    let str2 = str1;
    // println!("str1={}", str1); // 读取异常,str1已经被借走？
    println!("str2={}", str2);

    str3 += " hi";
    println!("str3={}", str3);

    // 传入 str3 的引用
    refer_test(&mut str3);
    println!("str3_refer={}", str3);
}

// 传入引用数据
fn refer_test(s: &mut String) {
    s.push_str(" world");
    // s += " world"; // 不能这样操作
    println!("s={}", s);
}

// 结构体
struct Point {
    x: i32,
    y: i32,
}
fn struct_test() {
    let mut p = Point { x: 1, y: 2 };
    println!("p.x={},p.y={}", p.x, p.y);
    change_point(&mut p);
    println!("p.x={},p.y={}", p.x, p.y);
    println!("get_area={}", get_area(&p));
}
fn change_point(p: &mut Point) {
    p.x = 10;
    p.y = 10;
}

fn get_area(p: &Point) -> i32 {
    return p.x * p.y;
}

// 枚举
#[derive(Debug)]
enum IpAddrKind {
    V4 = 4,
    V6 = 6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn enum_test() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    // 打印的kind并不是4和6？
    println!("home={},kind={:?}", home.address, home.kind);
    println!("loopback={},kind={:?}", loopback.address, loopback.kind);
}

// 模式匹配
fn match_test() {
    let x: i32 = 10;
    match x {
        1 => println!("one"),
        2 | 3 | 4 => println!("two"),
        5..=10 => println!("five"), // 左右闭区间
        _ => println!("other"),     // 默认匹配保证所有情况都被匹配
    }
}

// enum+match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 迭代器+闭包
fn iterator_test() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled={:?}", doubled);
}
