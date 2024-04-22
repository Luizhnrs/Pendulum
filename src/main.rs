fn main() {
    println!("Hello, world!");
}

struct Pendulum {
    //This is the angle of the pendulum
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r:f32, //The length of the pendulum
    m:f32, //The mass of the ball
    g:f32, //The gravity
}

