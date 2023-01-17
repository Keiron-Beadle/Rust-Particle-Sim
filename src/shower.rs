use crate::Vertex;
use glium::{Program, DrawParameters, Frame, Display, VertexBuffer, IndexBuffer, Surface};

pub struct Shower{
    position: [f32;3],
    vb:VertexBuffer<Vertex>, 
    ib: IndexBuffer<u16>
}

const INDICES: [u16;36] = [
    0,1,2,
    2,3,0,
    4,0,3,
    3,7,4,
    5,4,7,
    7,6,5,
    1,5,6,
    6,2,1,
    4,5,1,
    1,0,4,
    3,2,6,
    6,7,3
];

impl Shower{
    pub fn new(display : &Display)->Shower{
        let v1 = Vertex { position: [1.0, 1.0, 1.0], colour:[0.529, 0.301, 0.192] };
        let v2 = Vertex { position: [ -1.0,  1.0, 1.0], colour:[0.529, 0.301, 0.192] };
        let v3 = Vertex { position: [ -1.0, -1.0, 1.0], colour:[0.529, 0.301, 0.192] };
        let v4 = Vertex { position: [ 1.0, -1.0, 1.0], colour:[0.529, 0.301, 0.192] };
        let v5 = Vertex { position: [ 1.0, 1.0, -1.0], colour:[0.529, 0.301, 0.192] };
        let v6 = Vertex { position: [ -1.0, 1.0, -1.0], colour:[0.529, 0.301, 0.192] };
        let v7 = Vertex { position: [ -1.0, -1.0, -1.0], colour:[0.529, 0.301, 0.192] };
        let v8 = Vertex { position: [ 1.0, -1.0, -1.0], colour:[0.529, 0.301, 0.192] };
        let shape = vec![v1,v2,v3,v4,v5,v6,v7,v8];
        Shower{
            position : [0.0,1.0,0.0],
            vb:glium::VertexBuffer::new(display, &shape).unwrap(),
            ib:glium::IndexBuffer::new(display,glium::index::PrimitiveType::LineStrip, &INDICES).unwrap()
        }
    }

    pub fn draw(&self, frame: &mut Frame, program : &Program, view : [[f32;4];4], proj :[[f32;4];4], params : &DrawParameters){
        let uniforms = uniform! {
            world:[
                [0.5,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,0.5,0.0],
                [self.position[0],self.position[1],self.position[2],1.0]
            ],
            view:view,
            projection:proj
        };
        frame.draw(&self.vb, &self.ib, &program, &uniforms, &params).unwrap();
    }
}