// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }
//
//     fn y(&self) -> &U {
//         &self.y
//     }
// }
//
// // restrictions placed on just instances of Point's with f32 types
// impl Point<f32, f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// generics relevant to the struct
impl<X1, Y1> Point<X1, Y1> {
    // generics relevant to the method
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    // let p = Point { x: 5, y: 5 };
    // println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
