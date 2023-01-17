#[macro_use]
extern crate glium;
extern crate rand;
mod vshader;
mod fshader;
mod vertex;
mod shower;
mod movement_manager;
mod collision_manager;
mod temperature_manager;
mod shower_head;
mod particle;
mod particle_system;
use glium::Surface;
use vertex::Vertex;
use particle::Particle;
use particle_system::ParticleSystem;
use shower::Shower;
use shower_head::ShowerHead;

fn main(){
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let sim_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &sim_loop).unwrap();
    let params = glium::DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };
    let program = glium::Program::from_source(&display,vshader::get(), fshader::get(), None).unwrap();
    let mut particle_system = ParticleSystem::new(85000, 1.0,1.0, &display);
    let shower = Shower::new(&display);
    let shower_head = ShowerHead::new(&display);
    //let cam_pos:[f32;3]= [0.0,1.0,-2.5]; //0.0, 1.0, -2.5
    let cam_pos:[f32;3]= [0.0,1.0,-2.5]; //0.0, 1.0, -2.5
    let cam_dir:[f32;3]= [0.0,0.0,1.0];
    let view_mat = make_view(&cam_pos, &cam_dir, &[0.0,1.0,0.0]);
    sim_loop.run(move |event, _, control_flow| {
        match event{
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput{device_id, input:kin, is_synthetic:synth
                } => {
                    let pressed = kin.state == glutin::event::ElementState::Released;
                    let key = match kin.virtual_keycode{
                        Some(key)=>key,
                        None=>return,
                    };
                    match key{
                        glutin::event::VirtualKeyCode::M => {
                            if pressed{
                                particle_system.invert_colours()
                            }
                        },
                        _=>(),
                    }
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        let delta_t = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(delta_t);
        let mut target = display.draw();
        target.clear_color(0.0,0.0,0.0,1.0);
        particle_system.do_frame();
        let proj_mat = make_projection(&target);
        particle_system.draw(&mut target, &program, view_mat, proj_mat, &params);
        shower.draw(&mut target, &program, view_mat, proj_mat, &params);
        shower_head.draw(&mut target, &program, view_mat, proj_mat, &params);
        target.finish().unwrap();
        //println!("Collision Counter: {}", particle_system.collisions());
    });
}

fn make_view(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4]{
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn make_projection(target : &glium::Frame)-> [[f32; 4]; 4]{
    let dim = target.get_dimensions();
    let projection = {
        let (width, height) = dim;
        let aspect_ratio = height as f32 / width as f32;
    
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
    
        let f = 1.0 / (fov / 2.0).tan();
    
        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };
    return projection;
}