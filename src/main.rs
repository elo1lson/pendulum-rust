use vector::Vector;


fn main(){
}


struct Pendulum{
    origin: Vector,
    position: Vector,
    angle: f32,
    angle_velocity: f32,
    angular_acceleration: f32,
    r: f32,
    m: f32,
    g: f32,
    }
impl Pendulum {
    fn new() {
        
    }
    fn update(){
        
    }
    fn draw(){
        
    }
}        
                    
mod vector{
    pub struct Vector{
        pub x: f32,
        pub y:f32,
    }
    
    impl Vector {
        pub fn new(x: f32, y:f32)-> Vector{
            Vector{x, y}
        }
        pub fn add(&mut self, other:Vector) -> &Vector{
            self.x += other.x;
            self.y += other.y;
            self
        }
        pub fn set(&mut self, x:f32, y:f32){
            self.x= x;
            self.y = y;
        }
    
}
    
}
        