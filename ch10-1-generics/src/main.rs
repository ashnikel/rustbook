fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    &largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pointu<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pointu<T, U> {
    fn mixup<V, W>(self, other: Pointu<V, W>) -> Pointu<T, W> {
        Pointu {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![32, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    println!("both_integer.x = {}", both_integer.x());
    let both_float = Point { x: 1.0, y: 4.0 };
    println!("both_float.x = {}", both_float.x());
    println!("both_integer distance_from_origin = {}"
            , both_float.distance_from_origin());

    let p1 = Pointu { x: 5, y: 10.4 };
    let p2 = Pointu { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}