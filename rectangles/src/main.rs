#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square( size: u32 ) -> Self {
        Self { width: size, height: size }
    }

    fn area( &self ) -> u32 {
        self.width * self.height
    }

    fn can_hold( &self, other: &Rectangle ) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let sq = Rectangle::square( 40 );
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!( 30 * scale ),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    //dbg!( &rect1 );
}

// fn area( rectangle: &Rectangle ) -> u32 {
//     rectangle.width * rectangle.height
// }
