fn main() {
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
        let width = 30;
        let height = 50;
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width, height)
        );
    }

    // With tuples
    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
        let rectangle = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rectangle)
        );
    }

    // With structs
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rectangle)
        );
    }
}
