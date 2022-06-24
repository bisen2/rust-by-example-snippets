/*
    Structs can be broken down into three categories:
    - Tuple structs (named tuples/tranditional product types)
    - C-stye structs
    - Unit structs, which are fieldless
*/

#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
// a C-style struct
struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(({}, {}), ({}, {}))", self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y)
    }
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: min_x, y: max_y },
        bottom_right: Point { x: max_x, y: min_y },
    } = rect;
    let width = max_x - min_x;
    let height = max_y - min_y;
    width * height
}

fn square(top_left: &Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x: top_left.x, y: top_left.y },
        bottom_right: Point { x: top_left.x + size, y: top_left.y - size}
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // structs also provide an update syntax to use the field value from another struct
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // structs can be deconstructed using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y:top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 1., y: 10. },
        bottom_right: Point { x: 10., y: 1. }
    };

    println!("The area of rectangle {} is {}", rect, rect_area(&rect));

    let size = 5.;
    let pt = Point { x: 0., y: size };
    let sq = square(&pt, size);
    println!("A square with top left corner {} and size {} is defined as {}", pt, size, sq);
}
