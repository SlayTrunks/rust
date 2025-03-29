
//struct Rectangle {
//    width:u32,
//    height:u32,
//}
//fn main() {
//    let rectangle1 = Rectangle{
//        width:100,
//        height:73,
//    };
//    println!("The area of rectangle is {} square pexels", area(&rectangle1));
//}
//fn area(rect:&Rectangle)->usize{
//
//    let area = rect.width * rect.height;
//    area.try_into().unwrap() //try_into().unwrap() converts u32 into usize
//}



// struct Rectangle {
//     width:u32,
//     height:u32,
// }
// impl Rectangle {
//     fn area(&self)->u32{ //&self means self:&Self
//         self.width*self.height
//     }
// }
// fn main(){
// 
//     let rect1 = Rectangle {
//         width:100,
//         height:20,
//     };
//     println!("the area of the rectangle is {}",rect1.area());
// }



struct Rectangle {
    width:u32,
    height:u32
}
impl Rectangle {
    fn _area(&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self,rect:&Rectangle)->bool{
        self.width > rect.width && self.height>rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main(){
  let rect1 = Rectangle{
        width:12,
        height:100
    };
  let rect2 = Rectangle{
    width:32,
    height:111
  };
  let x = rect2.can_hold(&rect1);
  let rect3 = Rectangle::square(33);
  println!("{},{}",rect3.width,rect3.height);
  println!("{}",x);

}
