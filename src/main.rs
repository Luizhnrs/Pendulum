use vector::Vector;

fn main() {
    println!("Hello, world!");
}

struct Pendulum {
    //This vector is the position of the pendulum
    origin: Vector,

    //This vector is the position of the ball
    position: Vector,

    //This is the angle of the pendulum
    angle: f32, 

    angular_velocity: f32,
    angular_acceleration: f32,

    r:f32, //The length of the pendulum
    m:f32, //The mass of the ball
    g:f32, //The gravity
}

impl Pendulum {
    fn new(x: f32, y: f32, r:f32) -> Pendulum {
        Pendulum { 
            //We need to set the origin of the pendulum.
            origin: vector::Vector::new(x,y), 

            //We'll set the position when we update the pendulum.
            //For now we'll set it to a default value.
            position: vector::Vector::new { x:  0.0, y: 0.0 }, 


            angle: 1.0, //we'll set the angle to 1.0 radian
            angular_velocity: 0.0, //The pendulum is not moving in the beginning
            angular_acceleration: 0.0, //The pendulum is not accelerating is the beginning
            
            r: r, 
            m: 1.0, //The mass of the ball is 1.0 
            g: 1.5, //The gravity is 0.5, but play with it
        }
    }

    fn update($mut self){
        //We use the pendulum equation to calculate the angular acceleration
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration
        self.angular_velocity += self.angular_acceleration;

        //The position of the polar coordinate translated to cartesian coordinates
       
    }

    fn draw(){

    }
}

mod vector {


    pub struct Vector {
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

        pub fn set(&mut self, x: f32, y:f32) -> &Vector{ 
            self.x = x;
            self.y = y;
            self
        }
    }

}


