fn greet_world(){
    let test = "hello";
    let chinese = "你好";
    let english = "hello, world";
    let regions = [test, chinese, english];
    //我们没有使用其它语言惯用的 %s、%d 来做输出占位符，而是使用 {}，
    //因为 Rust 在底层帮我们做了大量工作，会自动识别输出数据的类型，例如当前例子，会识别为 String 类型。
    for region in regions.iter(){
        println!("{}", &region);
    }
}

fn test(){
    let  str = "
    1,123
    4,456
    7,789
    ";

    let records = str.lines();
    for (i, record) in records.enumerate(){
        println!("{},{}",i,record);
        if i==0 || record.trim().len() == 0{
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions){
            eprintln!("debug:{:?}->{:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>(){
            println!("{},{}cm",name, length);
        }
    }
}

fn test_unchange(){
    let mut x = 1;
    let _y  = 2; //ignore unused variable
    x = 2;
}

struct Struct {
    e: i32,
}

fn test_destruct(){
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

const MAX_POINTS: u32 = 100_000;

fn test_shadow(){
    let x = 1;
    let x = x+1;
    let space = "  ";
    let space = space.len();
    println!("{}",space);
}

fn test_type(){
    let i  = 1; //default i32
    let y = 1.0; // default f64
    let y: f32 = 1.0;
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    let z = i+ (y as i32) ; // nan different type add
    println!("{}",z);
    for i in 1..=5 {
        println!("{}",i);
    }
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}
// must set type for input variable 
fn test_return(x: i32, y: i32) -> i32{
    let x = x+1;
    let y = y+1; // 
    x+y // catious without ; , is expression used for return 
}
//单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：
//函数没有返回值，那么返回一个 ()
//通过 ; 结尾的表达式返回一个 ()

//当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
  }

//通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查
// int* foo() {
//     int a;          
//     a = 100;
//     char *c = "xyz";  //xyz不会释放
//     return &a;  //悬空指针
// }   
/*
1.Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
2.一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
3.当所有者(变量)离开作用域范围时，这个值将被丢弃(drop) */  

//let s ="hello" 字符串字面值 类型为 &str 不可变
//let s = String::from("hello"); 动态字符串类型: String 分配在堆区
//
fn test_string(){
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`
}
// String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量
//let s1 = String::from("hello");
//let s2 = s1;
/*(栈上拷贝复制，堆上所有权转移(c++中的move))
let s2 = s1;
1. 拷贝 String 和存储在堆上的字节数组 如果该语句是拷贝所有数据(深拷贝)，
那么无论是 String 本身还是底层的堆上数据，都会被全部拷贝，这对于性能而言会造成非常大的影响

2.只拷贝 String 本身 这样的拷贝非常快，因为在 64 位机器上就拷贝了 8字节的指针、
8字节的长度、8字节的容量，总计 24 字节，但是带来了新的问题，
还记得我们之前提到的所有权规则吧？其中有一条就是：一个值只允许有一个所有者，
而现在这个值（堆上的真实字符串数据）有了两个所有者：s1 和 s2。 
就假定一个值可以拥有两个所有者，会发生什么呢？

当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。
不过由于两个 String 变量指向了同一位置。这就有了一个问题：当 s1 和 s2 离开作用域，
它们都会尝试释放相同的内存。这是一个叫做 二次释放（double free） 的错误，
也是之前提到过的内存安全性 BUG 之一。两次释放（相同）内存会导致内存污染，
它可能会导致潜在的安全漏洞。

因此，Rust 这样解决问题：当 s1 赋予 s2 后，Rust 认为 s1 不再有效，
因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，
s1 在被赋予 s2 后就马上失效了。*/
fn test_string_copy(){
    let s1 = String::from("hello"); // s1 cannot used
    // let s2 = s1;
    /*
    let s1 = String::from("hello");
    let s2 = s1.clone();//深拷贝
    println!("s1 = {}, s2 = {}", s1, s2); */
    println!("{}, world!", s1);

}
/*
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作 */

/* 引用&（& 符号即是引用，它们允许你使用值，但是不获取所有权） 解引用* 
不可变引用
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
变量默认不可变一样，引用指向的值默认也是不可变的

可变引用
可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用
可变引用与不可变引用不能同时存在
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}
引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，
这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
   let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
}// 新编译器中，r3作用域在这里结束
在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你获取数据的引用后，
编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。
*/

/* slice 字符串切片的类型标识是 &str， let s: &str = "Hello, world!";
 */
fn test_slice(){
    let s = String::from("01234567");
    let s1 = &s[0..5]; //左闭右开 [1..]到最后一个字符为止 [..3]从0开始
    println!("{}",s1);
}
/*将 String 类型转为 &str 类型呢？
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str()); */

fn test_str_operation(){
    let mut s = String::from("hello,");
    s.push_str("world");
    s.insert_str(1,"!"); //操作现有的字符串
    let stemp = s.replace("hello","world");//返回新的字符串
    /*
    pop()删最后一个字符串，remove()删除字符串指定位置，
    truncate()删除字符串指定位置到结尾，clear()清空字符串

    使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型
    + 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。

    fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
    }
    format!("{} {}!", s1, s2);
    }
    */
}
fn test_tuple(){
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    let a = tuple.0;
}
/*  */
fn test_struct(){
    
}
fn main() {
    //greet_world();
    // test();
    // test_shadow();
    // test_type();
    test_slice();
}
