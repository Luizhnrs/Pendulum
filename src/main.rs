fn main() {
    println!("Hello, world!");
}

struct Pendulum {
    //This vector is the position of the pendulum
    origin: vector::Vector,

    //This vector is the position of the ball
    position: vector::Vector,

    //This is the angle of the pendulum
    angle: f32, 

    angular_velocity: f32,
    angular_acceleration: f32,

    r:f32, //The length of the pendulum
    m:f32, //The mass of the ball
    g:f32, //The gravity
}

impl Pendulum {
    fn new(){

    }

    fn update(){

    }

    fn draw(){

    }
}

mod vector {
    use std::io::SeekFrom;


    struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x:f32, y:f32) -> Vector {
            Vector{ x, y }
        }

        pub fn add(&mut self, other: Vector) -> &Vector{
            self.x += other.x;
            self.y += other.y;
            self
         }

        pub fn set(&mut self, x: f32, y:f32){ 
            self.x = x;
            self.y = y;
        }
    }

}


