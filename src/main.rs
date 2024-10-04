pub struct Formula{
    y_name: String,
    x_name: String,
    x: i32,
    b: i32,
}
struct Size {
    // size of coordinate plane
    pub size_x: i32,
    pub size_y: i32,
}
pub struct Plane {
    x: i32,
    y: i32,
    size: Size,
}
impl Plane {
    pub fn Render(func: String, size: Size) -> String {
        return func.clone();
    }
}

fn main() {
    println!("Hello, world!");
}
