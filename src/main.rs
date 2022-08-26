#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    // setup
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // settings up glium structs
    {
        use glium::implement_vertex;
        implement_vertex!(Vertex, position);
    }
    // setup buffers
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // setup shaders
    let vertsrc = include_str!("./vert.glsl");
    let fragsrc = include_str!("./frag.glsl");
    let program = glium::Program::from_source(&display, vertsrc, fragsrc, None).unwrap();

    let mut angle: f64 = 0.0; // type is important bcs float

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glium::glutin::event::Event::WindowEvent {event,..} => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
        angle += 0.0002;
        let mut target = display.draw();
        // drawing
        {
            use glium::Surface;
            use glium::uniform;
            let uniforms = &uniform! {
                matrix: [
                    [angle.cos() as f32, angle.sin() as f32, 0.0, 0.0],
                    [-angle.sin() as f32, angle.cos() as f32, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ]
            };
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(
                &vertex_buffer, &indices, &program, 
                uniforms, &Default::default()).unwrap();
        }
        target.finish().unwrap();
    })
}
