use std::thread::Thread;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use crate::{Particle};
//let mut thread_pool = scoped_threadpool::Pool::new((num_of_threads) as u32);

pub struct TemperatureManager{
    particles:Arc<Vec<Arc<Mutex<Vec<Particle>>>>>,
}

impl TemperatureManager{
    pub fn new(particles : Arc<Vec<Arc<Mutex<Vec<Particle>>>>>)->TemperatureManager{
        let tm = TemperatureManager{
            particles:particles,
        };
        return tm;
    }
}

pub fn TemperatureBeginWaiting(tm: &mut TemperatureManager, finished_transmitter : Sender<bool>, receiver : Receiver<usize>){
    let mut threadpool = scoped_threadpool::Pool::new(1);
    threadpool.scoped(|scope| {
        loop{
            let index1 = receiver.recv().unwrap();
            let strip1 = tm.particles[index1].clone();
            let index2 = receiver.recv().unwrap();
            let strip2 = tm.particles[index2].clone();
            scope.execute(move|| temperature_function(strip1));
            scope.execute(move|| temperature_function(strip2));
            scope.join_all();
            if index2 == tm.particles.len()-1{
                finished_transmitter.send(true).unwrap();
            }
        }
    });
}

fn temperature_function(data: Arc<Mutex<Vec<Particle>>>){
    let mut particles = data.lock().unwrap();
    for i in 0..particles.len(){
        particles[i].cooldown();
    }
}