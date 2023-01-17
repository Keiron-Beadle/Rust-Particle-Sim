use std::thread::Thread;
use std::{sync::{atomic::{AtomicI32}}};
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use crate::{Particle};
//let mut thread_pool = scoped_threadpool::Pool::new((num_of_threads) as u32);

const COLLISION_RADII :f32 = crate::particle_system::PARTICLE_SIZE * 2.0;

pub struct CollisionManager{
    particleStrips: Arc<Vec<Arc<Mutex<Vec<Particle>>>>>,
    bounding_width : f32,
    bounding_depth : f32,
    collision_count: Arc<AtomicI32>
}

impl CollisionManager{
    pub fn new(bounds_width: f32, bounds_depth : f32, particles : Arc<Vec<Arc<Mutex<Vec<Particle>>>>>)->CollisionManager{
        let cm = CollisionManager{
            particleStrips:particles,
            bounding_width : bounds_width,
            bounding_depth : bounds_depth,
            collision_count: Arc::new(AtomicI32::new(0))
        };
        return cm;
    }
}

pub fn CollisionBeginWaiting(cm : CollisionManager, transmitter : Sender<usize>, receiver : Receiver<usize>){
    let mut threadpool = scoped_threadpool::Pool::new(2);
    let width = cm.bounding_width;
    let depth = cm.bounding_depth;
    threadpool.scoped(|scope|{
        loop{
            let index1 = receiver.recv().unwrap();
            let strip1 = cm.particleStrips[index1].clone();
            let collision_copy_1 = cm.collision_count.clone();
            let index2 = receiver.recv().unwrap();
            let strip2 = cm.particleStrips[index2].clone();
            let collision_copy_2 = cm.collision_count.clone();
            scope.execute(move || collide_function(strip1,collision_copy_1, width, depth));
            scope.execute(move || collide_function(strip2,collision_copy_2, width, depth));
            scope.join_all();
            transmitter.send(index1).unwrap();
            transmitter.send(index2).unwrap();
            if index2 == cm.particleStrips.len()-1{
                //println!("Total Collisions With Bottom: {}", cm.collision_count.load(std::sync::atomic::Ordering::Acquire));
            }
        }
    });
}

fn collide_function(data: Arc<Mutex<Vec<Particle>>>, collision_count:Arc<AtomicI32>, width : f32, depth : f32){
    let mut particles = data.lock().unwrap();
    for i in 0..particles.len(){
        if !&particles[i].is_active(){
           continue;
        }
        let pos = particles[i].get_position();
        if pos[0] > 0.5 || pos[0] < -0.5{
           particles[i].invert_x_velocity();
        }
        if pos[1] < 0.0{
            //particles[i].invert_y_velocity();
            particles[i].deactivate(i);
            //inline function here to respawn particles
            let colliding_particle = &mut particles[i].to_owned();
            let particles_to_respawn = colliding_particle.get_collided_with();
            for i in 0..particles_to_respawn.len(){
                particles[particles_to_respawn[i]].activate();
            }
            collision_count.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            particles[i].activate();
        }
        if pos[2] > 0.5 || pos[2] < -0.5{
           particles[i].invert_z_velocity();
        }

        let (a,b) = particles.split_at_mut(i+1);
        let mut removes:Vec<usize> = Vec::new();
        let a_length = a.len();
        let particle = &mut a[a_length-1];
        for j in 0..b.len(){
            if !&b[j].is_active(){
                continue;
            }
            let particle2 = &mut b[j];
            if particle.distance_and_resolve(particle2, COLLISION_RADII, a_length+j){
                removes.push(j+a_length);
                break;
            }
        }
        for p in 0..removes.len(){
            particles[removes[p]].deactivate(removes[p]);
        }
    }
}