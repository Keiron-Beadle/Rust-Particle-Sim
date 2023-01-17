use std::{sync::{atomic::{AtomicI32}, Arc, Mutex}};
use glium::{Program, DrawParameters, Frame, Display, VertexBuffer, IndexBuffer, Surface};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::thread::JoinHandle;
use crate::{Particle, movement_manager::MovementManager, collision_manager::{CollisionManager}, temperature_manager::TemperatureManager};
use crate::Vertex;
use crate::collision_manager::CollisionBeginWaiting;
use crate::temperature_manager::TemperatureBeginWaiting;
pub const PARTICLE_SIZE :f32= 0.005;
const CONE_SPAWN_AREA : f32 = 0.1;

pub struct ParticleSystem{
    particles: Arc<Vec<Arc<Mutex<Vec<Particle>>>>>,
    movement: MovementManager,
    temperature_receiver : Receiver<bool>,
    collision : JoinHandle<()>,
    temperature :JoinHandle<()>,
    bound_width: f32,
    bound_depth: f32,
    inverted_colours:bool,
    vb:VertexBuffer<Vertex>, 
    ib: IndexBuffer<u16>
}

impl ParticleSystem{
    pub fn new(num_particles:usize, width:f32,depth:f32, display: &Display)->ParticleSystem{
        let shape = vec!(
            Vertex { position: [1.0, 1.0, 1.0], colour:[0.07,0.784,0.992] },
            Vertex { position: [ -1.0,  1.0, 1.0], colour:[0.07,0.784,0.992] },
            Vertex { position: [ -1.0, -1.0, 1.0], colour:[0.07,0.784,0.992] }
        );
        let temp = Particle::new(10.0);
        let num_strips = 12;
        //let num_threads = 6;
        let len = num_particles / num_strips;
        let mut temp_strips: Vec<Arc<Mutex<Vec<Particle>>>> = Vec::new();
        
        for _ in 0..num_strips{
            let mut particles :Vec<Particle>= Vec::new();
            for _ in 0..len{
                let new_particle = Particle::new(CONE_SPAWN_AREA);
                particles.push(new_particle);
            }
            let arc = Arc::new(Mutex::new(particles));
            temp_strips.push(arc.clone());
        }
        let strips = Arc::new(temp_strips);
        let move_particles = strips.clone();
        let collision_particles = strips.clone();
        let temp_particles = strips.clone();
        let (tmc, rmc) : (Sender<usize>, Receiver<usize>)= mpsc::channel(); //TransmitterMovementCollision, ReceiverMovementCollision
        let (tct, rct) : (Sender<usize>, Receiver<usize>)= mpsc::channel(); //TransmitterCollisionTemperature, ReceiverCollisionTemperature
        let (temp_finished_transmitter, temp_finished_receiver) : (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let movement_manager = MovementManager::new(move_particles, tmc);
        let collision_manager = CollisionManager::new(width, depth, collision_particles);
        let mut temperature_manager = TemperatureManager::new(temp_particles);
        let collision_join_handle = std::thread::spawn(move || CollisionBeginWaiting(collision_manager, tct, rmc));
        let temperature_join_handle = std::thread::spawn(move || TemperatureBeginWaiting(&mut temperature_manager,temp_finished_transmitter,rct));
        ParticleSystem{
            particles:strips,
            movement:movement_manager,
            collision :collision_join_handle,
            temperature : temperature_join_handle,
            temperature_receiver : temp_finished_receiver,
            bound_width : width,
            bound_depth : depth,
            inverted_colours:false,
            vb:glium::VertexBuffer::dynamic(display, &shape).unwrap(),
            ib:glium::IndexBuffer::new(display,glium::index::PrimitiveType::TrianglesList, &temp.get_indices()).unwrap()
        }
    }

    pub fn invert_colours(&mut self){
        self.inverted_colours = !self.inverted_colours;
    }

    pub fn do_frame(&mut self){
        self.movement.do_frame();
        self.temperature_receiver.recv().unwrap(); //Blocks until get message from temperature manager
    }

    pub fn draw(&mut self, frame: &mut Frame, program : &Program, view : [[f32;4];4], proj :[[f32;4];4], params : &DrawParameters){
        //Only draw first strip
        let particle_clone = self.particles[0].clone();
        let particle_strip = particle_clone.lock().unwrap();
        let num_particles_to_draw = std::cmp::min(particle_strip.len(), 500);
        for i in 0..num_particles_to_draw
        {
            if !&particle_strip[i].is_active(){
                continue;
            }
            let pos = particle_strip[i].get_position(); //Get current particle position
            let col : [f32;3];
            if !self.inverted_colours{
                col = particle_strip[i].get_temperature_colour();
            }
            else{
                col = particle_strip[i].get_mass_colour();
            }            
            let shape =  vec!( //Upload current particle's colour to VBO
                Vertex { position: [1.0, 1.0, 1.0], colour:col },
                Vertex { position: [ -1.0,  1.0, 1.0], colour:col },
                Vertex { position: [ -1.0, -1.0, 1.0], colour:col }
            );
            self.vb.write(&shape); //Upload vbo data
            let uniforms = uniform! {
                world:[
                    [PARTICLE_SIZE,0.0,0.0,0.0],
                    [0.0,PARTICLE_SIZE,0.0,0.0],
                    [0.0,0.0,PARTICLE_SIZE,0.0],
                    [pos[0],pos[1],pos[2],1.0]
                ],
                view:view,
                projection:proj
            };
            frame.draw(&self.vb, &self.ib, &program, &uniforms, &params).unwrap();
        }
    }
}