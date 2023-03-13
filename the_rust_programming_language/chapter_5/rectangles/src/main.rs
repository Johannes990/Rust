#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle      // here, in the impl block we denife methods and associated functions for structs
{
    fn square(size: u32) -> Rectangle       // overwritten constructor for a single data input
    {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn diagonal(&self) -> f32
    {
        let sum_of_squares: f32 = (self.width.pow(2) + self.height.pow(2)) as f32;
        sum_of_squares.sqrt()
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

fn main()
{
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 87, height: 116 };
    let rect3 = Rectangle { width: 25, height: 25};
    let square1 = Rectangle::square(25);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The length of the diagonal of the rectangle is {} pixels.",
        rect1.diagonal()
    );

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect1? {}\n", rect3.can_hold(&rect1));

    println!("rect1 area: {}, rect1 diagonal: {}", rect1.area(), rect1.diagonal());
    println!("rect2 area: {}, rect2 diagonal: {}", rect2.area(), rect2.diagonal());
    println!("rect3 area: {}, rect3 diagonal: {}", rect3.area(), rect3.diagonal());

    println!("Rectangle is: {:#?}", rect1);

    println!("The square is: {:#?}", square1);
}