use crate::Vertex;
use crate::glium::Display;
use rand::{thread_rng, Rng};

implement_vertex!(Vertex, position, colour);

const GRAVITY : f32 = -0.681; //-0.981
const COOL_FACTOR:f32 = 0.6;

#[derive(Clone,Debug)]
pub struct Particle{
    velocity: [f32;3],
    position: [f32;3],
    mass:f32,
    collided_with:Vec<usize>,
    active:bool,
    spawn_bounds:f32,
    temperature:f32
}

const INDICES: [u16;3] = [
    0,1,2
];

impl Particle{
    pub fn new(spawn_bounds : f32)->Particle{
        let mut rng = thread_rng();
        let x = rng.gen_range( -spawn_bounds, spawn_bounds);
        //let y = rng.gen_range( -spawn_bounds, spawn_bounds) + 1.0;
        let y = 1.95;
        let z = rng.gen_range(-spawn_bounds , spawn_bounds );
        //let z = -0.2;
        //30 degrees X rotation 0, 0.0086603, 0.005
        //-30 degrees ^^ 0, 0.0086603, -0.005
        // 30 degrees Z rotation -0.005, 0.0086603, 0
        //-30 degrees ^^ 0.005,0.0086603, 0
        let xv = rng.gen_range(-0.005, 0.005); // 0.005
        let yv = -0.01663303;
        let zv = rng.gen_range(-0.005, 0.005);
        // let xv = rng.gen_range(0.001, 0.005);
        // let yv =rng.gen_range(0.001, 0.005);
        //let zv = 0.0;
        Particle{
            velocity : [xv,yv,zv],
            position : [x,y,z],
            mass:1.0,
            collided_with: Vec::new(),
            active: true,
            spawn_bounds:spawn_bounds,
            temperature: 1.0
        }
    }

    //False means no collision, True means there was a collision 
    pub fn distance_and_resolve(&mut self, other:&mut Particle, sum_radii : f32, other_index:usize) -> bool{
        let result_x = self.position[0] - other.position[0];
        let result_y = self.position[1] - other.position[1];
        let result_z = self.position[2] - other.position[2];
        let dist = (result_x*result_x) + (result_y*result_y) + (result_z*result_z);
        if dist < sum_radii * sum_radii{
            let mass_p1 = self.mass;
            let mass_p2 = other.mass;
            let x_component = (mass_p1 * self.velocity[0]) + (mass_p2 as f32 * other.velocity[0]);
            let y_component = (mass_p1 * self.velocity[1]) + (mass_p2 as f32 * other.velocity[1]);
            let z_component = (mass_p1 * self.velocity[2]) + (mass_p2 as f32 * other.velocity[2]);
            let denominator = mass_p1 + mass_p2;
            self.velocity[0] = x_component / denominator;
            self.velocity[1] = y_component / denominator;
            self.velocity[2] = z_component / denominator;
            self.mass += mass_p2;
            other.mass = 0.0;
            self.collided_with.append(&mut other.collided_with);
            self.collided_with.push(other_index);
            return true;
        }
        return false;
    }

    pub fn get_collided_with(&mut self)->&mut[usize]{
        return self.collided_with.as_mut_slice();
    }

    pub fn get_mass(&self)->f32{
        return self.mass;
    }

    pub fn get_velocity(&self)->[f32;3]{
        return self.velocity;
    }

    pub fn is_active(&self)->bool{
        return self.active;
    }

    pub fn activate(&mut self){
        let mut rng = thread_rng();
        let random_x = rng.gen_range(-self.spawn_bounds, self.spawn_bounds);
        //let random_y = rng.gen_range(-self.spawn_bounds, self.spawn_bounds) + 1.0;
        let random_z = rng.gen_range(-self.spawn_bounds, self.spawn_bounds);
        //let random_z = -0.2;
        let xv = rng.gen_range(-0.005, 0.005);
        let yv = -0.00863303;
        let zv = rng.gen_range(-0.005, 0.005);
        //let zv = 0.0;
        self.position = [random_x, 2.0, random_z];
        self.velocity = [xv,yv,zv];
        self.mass = 1.0;
        self.temperature = 1.0;
    }

    pub fn deactivate(&mut self, index:usize){
        self.position = [index as f32,1000.0,1000.0];
        self.velocity = [0.0,0.0,0.0];
    }

    pub fn move_particle(&mut self){
        self.position[0] += self.velocity[0];
        //self.position[1] += self.velocity[1];
        self.position[1] += self.mass * GRAVITY * 0.0166667;
        self.position[2] += self.velocity[2];
    }

    pub fn invert_x_velocity(&mut self){
        self.velocity[0] = -self.velocity[0];
    }
    pub fn invert_y_velocity(&mut self){
        self.velocity[1] = -self.velocity[1];
    }
    pub fn invert_z_velocity(&mut self){
        self.velocity[2] = -self.velocity[2];
    }

    pub fn cooldown(&mut self){
        self.temperature = self.temperature - (0.016666 * COOL_FACTOR / self.mass);
    }
    
    pub fn get_mass_colour(&self)->[f32;3]{
        let factor_val = self.mass / 5.0;
        let red_component = factor_val;
        let blue_component = 1.0-factor_val;
        return [red_component, 0.0, blue_component];
    }

    pub fn get_temperature_colour(&self)->[f32;3]{
        let red_component = self.temperature;
        let blue_component = 1.0-self.temperature;
        return [red_component, 0.0, blue_component];
    }

    pub fn get_indices(&self)->[u16;3]{
        return INDICES;
    }

    pub fn get_position(&self)->[f32;3]{
        return self.position;
    }
}