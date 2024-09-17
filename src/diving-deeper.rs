// Diving Deeper - Vectors

// #![allow(unused)]
// fn main() {
//     // let mut vec = Vec::new();
//     // vec.push(1);
//     // vec.push(2);
//
//     let mut vec = vec![1,2];
//
//     println!("len: {}", vec.len());
//     println!("first element: {}", vec[0]);
// }

// Diving Deeper - Enums
// #[allow(unused)]
// enum Fruit {
//     Banana(String),
//     Apple(String),
//     Orange(String)
// }
//
// #[allow(unused)]
// fn main() {
// let banana = Fruit::Banana("ripe".to_string());
//
// match banana {
//     Fruit::Banana(adj) => println!("banana: {}", adj),
//     Fruit::Apple(adj) => println!("banana: {}", adj),
//     Fruit::Orange(adj) => println!("banana: {}", adj),
//     _ => println!("No fruit")
// }

// if let Fruit::Banana(adj) =  banana {
//     println!("banana adj is: {}", adj);
// } else {
//     println!("not banana!");
// }

//
//
// let x: Result<i32, &str> = Ok(-3);
// let y: Result<i32, &str> = Err("error");
// println!("The result is ok: {}", x.is_ok());
// println!("The result is err: {}", y.is_ok());



// let z: i32 = 5; // you cannot do this!
// print("the sum of x and z: {}", x+y) // you cannot do this!
// }

// EXTRA PRACTICE

// enum Point {
//     Nothing,
//     TuplePoint(i32, i32),
//     StructPoint {
//         x: i32,
//         y: i32
//     }
// }
//
// fn get_point(n: u8) -> Point {
//     match n {
//         1 => Point::TuplePoint(-1, 1),
//         2 => Point::StructPoint {
//             x: -1,
//             y: 1
//         },
//         _ => Point::Nothing
//     }
// }
//
// fn main() {
//     let p = get_point(2);
//     match p {
//         Point::Nothing => println!("no point"),
//         Point::TuplePoint(x, y) => println!("x is {} and y is {}", x, y),
//         Point::StructPoint{x, y} => println!("x is {} and y is {}", x, y),
//     }
// }
