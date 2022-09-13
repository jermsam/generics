use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");

    let my_arr:[i8;7] = [2,4,7,5,8,5,0];
    let largest_in_my_arr:i8 = largest(&my_arr);
    println!("Largest item in {:?} is {}",my_arr, largest_in_my_arr);
    let mut my_vec:Vec<i8> = vec![2, 4, 7, 5, 8, 5, 0];
    my_vec.push(56);
    let largest_in_my_vec:i8 = largest(&my_vec);
    println!("Largest item in {:?} is {}",my_vec, largest_in_my_vec);

    let pt = Point::new(&5,&0.8);
    let pt2 = Point::new(&0.5,&8);
    let mix_pt = pt.mix_up(&pt2);
    println!("Crazy 8: {:?}",mix_pt);

    let pol:PolygonType<(i32,i32)> = PolygonType::Rectangle((7,2));
    println!("Crazy T: {:?}",pol);
    let pol:PolygonType<i32> = PolygonType::Square(2);
    println!("Crazy T: {:?}",pol);
    let pol = PolygonType::Rectangle((mix_pt,pt2));
    println!("Crazy TP: {:?}",pol);
}

// generic function to find largest item in a list
fn largest<T:  PartialOrd + Copy+ Debug + Display>(list: &[T]) -> T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    *largest
}

#[derive(Debug)]
struct Point<X:Copy, Y:Copy> {
    x: X,
    y: Y,
}

impl<U: Copy, V: Copy> Point<U, V> {
    fn new (a: U, b: V) -> Point<U, V> {
        Self {
            x:a,
            y:b
        }
    }
    fn mix_up<W: Copy, Z: Copy> (&self, other: &Point<W, Z>) -> Point<U, Z> {

        Point::new(self.x,other.y)

    }
}


#[derive(Debug)]
enum PolygonType<T> {
    Rectangle(T),
    Square(T),
}




