use std::thread::{Thread};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc,Mutex};
use glium::index;
use std::thread::sleep;
use std::thread::JoinHandle;
use crate::{Particle};
//let mut thread_pool = scoped_threadpool::Pool::new((num_of_threads) as u32);

pub struct MovementManager{
    particleStrips: Arc<Vec<Arc<Mutex<Vec<Particle>>>>>,
    transmitter: Sender<usize>
}

impl MovementManager{
    pub fn new(particles : Arc<Vec<Arc<Mutex<Vec<Particle>>>>>, trans: Sender<usize>)->MovementManager{
        MovementManager{
            particleStrips:particles,
            transmitter:trans
        }
    }

    pub fn do_frame(&mut self){
        let mut threadpool = scoped_threadpool::Pool::new(2);
        threadpool.scoped(|scope|{
            let length = self.particleStrips.len() / 2 ;
            for i in 0..length{
                let index1 = i*2;
                let index2 = index1+1;
                let strip = self.particleStrips[index1].clone();
                let strip2 = self.particleStrips[index2].clone();
                scope.execute(move || move_function(strip, index1));
                scope.execute(move || move_function(strip2, index2));
                scope.join_all();
                self.transmitter.send(index1).unwrap();
                self.transmitter.send(index2).unwrap();
            }
        });
    }
}

fn move_function(data: Arc<Mutex<Vec<Particle>>>, index : usize){
    let mut particles = data.lock().unwrap();
    for i in 0..particles.len(){
        //println!("V: {:?}", particles[i].get_velocity());
        // let vel = particles[i].get_velocity();
        // if index == 0{
        //    // println!("MOV VEL: {:?}",vel);
        // }
        particles[i].move_particle();

    }
}