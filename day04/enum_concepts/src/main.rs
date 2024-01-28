// Create an enum called Shape and provide the values of "triangle square, pentagon, octagon".  Then create a method for this enum that returns the number of corners each shape has based on the type of shape.

enum Shape { triangle, square, pentagon, octago }

impl Shape {
    
    fn corners(self) -> String {
        match self {
            Shape::octago => "8 corners".to_string(),
            Shape::triangle => "3 corners".to_string(),
            Shape::square => "4 corners".to_string(),
            Shape::pentagon => "5 corners".to_string(),

        }
    }

}
fn main()
{
    let type1 = Shape::triangle;
    println!("{}", type1.corners());
}