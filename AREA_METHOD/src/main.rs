// struct Object {
//     width: u32,
//     height: u32,
// }

// impl Object {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn new(width: u32, height: u32) -> Object {
//         Object{
//             width,
//             height,
//         }
//     }

//     fn show(&self) {
//         println!("{}x{} with area: {}", self.width, self.height, self.area());
//     }
// }

// fn main() {
//     let o = Object {
//         width : 10,
//         height : 15,
//     };

//     // let j = Object {
//     //     width: 13,
//     //     height: 11,
//     // };

//     // println!("Area is {}", o.area());
//     // println!("Area is {}", j.area());

//     let j: Object = Object::new(57, 83);

//     o.show();
//     j.show();
// }


struct Triangle {
    base: u32,
    height: u32,
}

impl Triangle {
    fn area(&self) -> f32 {
        self.base as f32 * self.height as f32 * 0.5
    }

    fn new(base: u32, height: u32) -> Triangle {
        Triangle{
            base,
            height,
        }
    }

    fn show(&self) {
        println!("Base: {}, Height: {} with area: {}", self.base, self.height, self.area());
    }
}

fn main() {
    let t: Triangle = Triangle::new(20, 5);

    t.show();
}