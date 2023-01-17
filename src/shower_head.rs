use crate::Vertex;
use glium::{Program, DrawParameters, Frame,Display, VertexBuffer, Surface};

pub struct ShowerHead{
    position:[f32;3],
    vb:VertexBuffer<Vertex>,
    ib:glium::index::NoIndices,
}

impl ShowerHead{
    pub fn new(display: &Display)->ShowerHead{
        //Make a nonagram
        let colour = [0.807, 0.6, 0.274];
        let start_point = [0.1,2.0,0.1];
        let shape:Vec<Vertex> =vec!(
            Vertex{position:start_point, colour:colour},
            Vertex{position:[0.140883, 2.0, 0.012326], colour:colour},
            Vertex{position:[0.115846, 2.0, -0.081116], colour:colour},
            Vertex{position:[0.036603, 2.0, -0.136603], colour:colour},
            Vertex{position:[-0.059767, 2.0, -0.128172], colour:colour},
            Vertex{position:[-0.128172, 2.0, -0.059768], colour:colour},
            Vertex{position:[-0.136604, 2.0, 0.036602], colour:colour},
            Vertex{position:[-0.081117, 2.0, 0.115846], colour:colour},
            Vertex{position:[0.012325, 2.0, 0.140884], colour:colour}
        );
        ShowerHead{
            position: [0.0,0.0,0.0],
            vb:glium::VertexBuffer::new(display, &shape).unwrap(),
            ib:glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan)
        }
    }

    pub fn draw(&self, frame:&mut Frame, program: &Program, view: [[f32;4];4], proj:[[f32;4];4], params: &DrawParameters){
        let uniforms = uniform! {
            world:[
                [1.0,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [self.position[0],self.position[1],self.position[2],1.0]
            ],
            view:view,
            projection:proj
        };
        frame.draw(&self.vb, &self.ib, &program, &uniforms, &params).unwrap();
    }
}