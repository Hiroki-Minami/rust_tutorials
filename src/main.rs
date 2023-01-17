// use std::collections::HashMap;

// use exercise::abc;
// use exercise::exercise_test::lib_test;

use std::fmt::{Display, Debug};

fn main() {
    // lab1()
    // lab2()
    // lab3()
    // lab4()
    // lab5()
    // lab6()
    // lab7()
    // fizzbuzz_to(20);
    // lab8()
    // lab9()
    // lab10()
    // lab11()
    // lab12()
    // lab13()
    // lab14_vec()
    // lab15()
    // lab16()
    // lab17()
    // lab18();
    // lab19();
    // exercise::exercise_test::lib_test();
    // lib_test();
    // abc();
    // lab22();
    // let first = pig_latin("first");
    // let apple = pig_latin("apple");
    // println!("{first}");
    // println!("{apple}");
    // add_employee("Engineering", "Sally");
    // lab24();
    lab25();
}

// fn lab1() {
//     // println!("Edit me!");
//     let mut x: i32 = 6;
//     print!("{x}");
//     while x != 1 {
//         if x % 2 == 0 {
//             x = x / 2;
//         } else {
//             x = 3 * x + 1;
//         }
//         print!(" -> {x}");
//     }
//     println!();
// }

// fn lab2() {
//     let mut a: [i8; 10] = [42; 10];
//     a[5] = 0;
//     println!("a: {:?}", a)
// }

// fn lab3() {
//     let t: (i8, bool) = (7, true);
//     println!("1st index: {}", t.0);
//     println!("2st index: {}", t.1);
// }

// fn lab4() {
//     let mut x: i32 = 10;
//     let ref_x: &mut i32 = &mut x;
//     *ref_x = 20;
//     println!("x: {x}");
// }

// fn  lab5() {
//     let ref_x: &i32;
//     {
//         let x: i32 = 10;
//         ref_x = &x;
//     }
//     println!("ref_x: {ref_x}")
// }

// fn lab6() {
//     let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     print!("a: {a:?}");

//     let s: &[i32] = &a[2..4];
//     println!("s: {s:?}");
// }

// fn lab7() {
//     let s1: &str = "Hello";
//     println!("s1: {s1}");

//     let mut s2: String = String::from("Hello ");
//     println!("s2: {s2}");
//     s2.push_str(s1);
//     println!("s2: {s2}");
// }

// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if lhs == 0 {
//         return false;
//     } 
//     return lhs % rhs == 0;
// }

// fn fizzbuzz(n: u32) -> () {
//     match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
//         (true, true) => println!("fizzbuzz"),
//         (true, false) => println!("fizz"),
//         (false, true) => println!("buzz"),
//         (false, false) => println!("{n}"),
//     }
// }

// fn fizzbuzz_to(n: u32) {
//     for n in 1..n  {
//         fizzbuzz(n);
//     }
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn inc_width(&mut self, delta: u32) {
//         self.width += delta;
//     }
// }

// fn lab8() {
//     let mut rec = Rectangle {width: 10, height: 5};
//     println!("old area: {}", rec.area());
//     rec.inc_width(5);
//     println!("new area: {}", rec.area());
// }

// fn pick_one<T>(a: T, b: T) -> T {
//     if std::process::id() % 2 == 0 {a} else {b}
// }

// fn lab9() {
//     println!("coin toss: {}", pick_one("heads", "tails"));
//     println!("cash prize: {}", pick_one(500, 1000));
// }

// fn multiply(x: i16, y: i16) -> i16 {
//     x * y
// }

// fn lab10() {
//     let x: i8 = 15;
//     let y: i16 = 1000;
//     println!("{x} * {y} = {}", multiply(x.into(), y));
// }

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     let mut ret_matrix: [[i32; 3]; 3] = [[0,0,0],[0,0,0],[0,0,0,]];
//     for i in 0..3 {
//         for j in 0..3 {
//             ret_matrix[i][j] = matrix[j][i];
//         }
//     }
//     return ret_matrix;
// }

// fn pretty_print(matrix: &[[i32; 3]; 3]) {
//     for i in matrix {
//         for j in i {
//             print!("{j} ");
//         }
//         println!()
//     }
// }

// fn lab11() {
//     let matrix = [
//         [101, 102, 103],
//         [201, 202, 203],
//         [301, 302, 303],
//     ];

//     println!("matrix:");
//     pretty_print(&matrix);

//     let transponsed = transpose(matrix);
//     println!("transposed:");
//     pretty_print(&transponsed);
// }

// const DIGENST_SIZE: usize = 3;
// const ZERO: Option<u8> = Some(42);

// static BANNER: &str = "Welcome to Rust";

// fn compute_digest(text: &str) -> [u8; DIGENST_SIZE] {
//     let mut digest = [ZERO.unwrap_or(0); DIGENST_SIZE];
//     for (idx, &b) in text.as_bytes().iter().enumerate() {
//         digest[idx % DIGENST_SIZE] = digest[idx % DIGENST_SIZE].wrapping_add(b);
//     }
//     digest
// }

// fn lab12() {
//     let digest = compute_digest("HHHl");
//     println!("digest: {digest:?}");
//     println!("{BANNER}")
// }

// #[derive(Debug)]
// struct Point(i32, i32);

// fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
//     if p1.0 < p2.0 {p1} else {p2}
// }

// fn lab13() {
//     let p1: Point = Point(10, 10);
//     let p2: Point = Point(20, 20);
//     let p3: &Point = left_most(&p1, &p2);
//     println!("left-most point: {:?}", p3);
//     println!("p1: {:?}", p1);
// }

// fn lab14_vec() {
//     let mut vec = vec![10, 20];
//     vec.push(30);
//     println!("middle value {}", vec[vec.len() / 2]);
//     for item in vec.iter()  {
//         println!("item: {item}")
//     }
// // }

// struct Book {
//     title: String,
//     year: u16,
// }

// struct Library {
//     books: Vec<Book>,
// }

// impl Book {
//     fn new(title: &str, year: u16) -> Book {
//         Book { title: String::from(title), year }
//     }
// }

// impl std::fmt::Display for Book {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ({})", self.title, self.year)
//     }
// }

// impl Library {
//     fn new() -> Library {
//         Library { books: vec![] }
//     }

//     fn len(&self) -> usize {
//         self.books.len()
//     }

//     fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     fn add_book(&mut self, book: Book) {
//         self.books.push(book);
//     }

//     fn print_books(&self) {
//         for book in &self.books {
//             println!("{book}");
//         }
//     }

//     fn oldest_book(&self) -> Option<&Book> {
//         if self.is_empty() { return None } 
//         let mut oldest_book_index: usize = 0;
//         for i in 1..self.len() {
//             if self.books[oldest_book_index].year > self.books[i].year {
//                 oldest_book_index = i;
//             }
//         }
//         Some(&self.books[oldest_book_index])
//     }
// }

// fn lab15() {
//     let mut library = Library::new();
//     println!("Our library is empty: {}", library.is_empty());
    
//     library.add_book(Book::new("Lord of the Rings", 1954));
//     library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
//     library.print_books();
    
//     match library.oldest_book() {
//        Some(book) => println!("My oldest book is {book}"),
//        None => println!("My library is empty!"),
//     }
    
//     println!("Our library has {} books", library.len());
// }

// fn lab16() {
//     let v: Vec<i8> = vec![10, 20, 30];
//     let mut iter = v.iter();

//     println!("v[0]: {:?}", iter.next());
//     println!("v[1]: {:?}", iter.next());
//     println!("v[2]: {:?}", iter.next());
//     println!("No more item {:?}", iter.next());
// }

// enum WebEvent {
//     PageLoad,
//     KeyPress(char),
//     Click {x: i64, y: i64},
// }

// #[rustfmt::skip]
// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded."),
//         WebEvent::KeyPress(c) => println!("pressed {c}"),
//         WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}")
//     }
// }

// fn lab17() {
//     let load = WebEvent::PageLoad;
//     let press = WebEvent::KeyPress('x');
//     let click = WebEvent::Click { x: 20, y: 80 };

//     inspect(load);
//     inspect(press);
//     inspect(click);
// }

// #[derive(Debug)]
// struct Race {
//     name: String,
//     laps: Vec<i32>,
// }

// impl Race {
//     fn new(name: &str) -> Race {
//         Race { name: String::from(name), laps: Vec::new() }
//     }

//     fn add_lap(&mut self, lap: i32) {
//         self.laps.push(lap);
//     }

//     fn print_laps(&self) {
//         println!("Recoded {} laps for {}", self.laps.len(), self.name);
//         for (idx, lap) in self.laps.iter().enumerate() {
//             println!("lap {idx}: {lap} sec");
//         }
//     }

//     fn finish(self) {
//         let total = self.laps.iter().sum::<i32>();
//         println!("Race {} is finished, total lap time: {}", self.name, total);
//     }
// }

// fn lab18() {
//     let mut race = Race::new("Monaco Grand Prix");
//     race.add_lap(70);
//     race.print_laps();
//     race.add_lap(80);
//     race.finish()
// }

// struct User {
//     name: String,
//     age: u32,
//     weight: f32,
// }

// impl User {
//     pub fn new(name: String, age: u32, weight: f32) -> Self {
//         User { name, age, weight }
//     }

//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     pub fn age(&self) -> u32 {
//         self.age
//     }

//     pub fn weight(&self) -> f32 {
//         self.weight
//     }

//     pub fn set_age(&mut self, new_age: u32) {
//         self.age = new_age
//     }

//     pub fn set_weight(&mut self, new_weight: f32) {
//         self.weight = new_weight
//     }
// }

// fn lab19() {
//     let mut bob = User::new(String::from("Bob"), 32, 155.2);
//     println!("I'm {} and my age is {}", bob.name(), bob.age());
//     bob.set_age(33);
//     println!("I'm {} and my age is {}", bob.name(), bob.age());
//     bob.set_weight(160.2);
//     println!("I'm {} and my age is {} my weight is {}", bob.name(), bob.age(), bob.weight());
// }

// fn lab20() {
//     let mut s1 = String::from("value");
//     let s2 = "add";
//     s1.push_str(s2);
//     println!("s1: {s1}");
// }

// fn lab21() {
//     use std::collections::HashMap;

//     let mut map = HashMap::new();
    
//     map.insert(String::from("key1"), 32);
//     map.insert(String::from("key2"), 64);

//     let v1 = map.get("key1").copied().unwrap_or(0);

//     let text = "Hello world wonderful world";

//     let mut map2 = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map2.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map2);
// }

// fn lab22() {
//     let median = return_median(&vec![1,2,3]);
//     println!("median: {}", median);
// }

// fn return_median(vec: &Vec<i32>) -> i32 {
//     let count  = vec.len();
//     match count {
//         0 => 0,
//         1 => vec[0],
//         other => {
//             if other % 2 == 1 {
//                 return vec[other / 2];
//             } else {
//                 return (vec[other / 2] + vec[(other / 2) - 1]) / 2;
//             }
//         }
//     }
// }

// fn pig_latin(str: &str) -> String {
//     let first_letter = str.chars().nth(0);
//     if let Some(fl) = first_letter {
//         match fl {
//             'a' | 'i' | 'e' | 'o' | 'u' =>  String::from(str) + "-hay",
//             _other => {
//                 let mut s = String::from(&str[1..str.len()]);
//                 s.push('-');
//                 s.push(fl);
//                 s.push_str("ay");
//                 s
//             },
//         }
//     } else {
//         String::new()
//     }
// }

// fn add_employee(department: &str, employee: &str) {
//     let mut map: HashMap<String, Vec<String>> = HashMap::new();

//     let vec = vec![String::from(employee)];
//     map.insert(String::from(department), vec);

//     for (k, v) in map {
//         for e in v {
//             println!("department: {k}, employee: {e}")
//         }
//     }
// }

// use std::fs::File;
// use std::io::{ErrorKind, Read};

// fn lab23() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening file: {:}?", other_error),
//         },
//     };
// }

// fn read_username_from_file() -> Result<String, std::io::Error> {
//     let user_name_file_result = File::open("hello.txt");

//     let mut username_file = match user_name_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }

//     // let mut username_file = user_name_file_result?;
//     // let mut username = String::new();
//     // username_file.read_to_string(&mut username)?;
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn lab24() {
//     let point = Point {x: 5, y: 10};
//     let p2: Point<f32> = Point {x: 3.2, y: 3.8};
//     println!("p.x = {}", point.x());
//     println!("{}", p2.distance_from_origin());
// }

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn lab25() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize())
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize())
}

// pub fn notify(item: &(impl Summary + Display)) {}

// pub fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug {
    
// }