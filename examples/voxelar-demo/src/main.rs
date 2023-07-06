extern crate voxelar;

use voxelar::glfw::*;
use voxelar::opengl::gl;
use voxelar::opengl::mesh::Mesh;
use voxelar::opengl::program::Program;
use voxelar::opengl::shader::*;
use voxelar::opengl::uniform::*;
use voxelar::opengl::vao::Vao;
use voxelar::opengl::vbo::Vbo;
use voxelar::opengl::GlContext;
use voxelar::receivable_events::*;
use voxelar::render_context::RenderContext;
use voxelar::voxelar_math::vec3::Vec3;
use voxelar::window::*;
use voxelar::*;

fn create_vert_shader() -> crate::Result<Shader> {
    shader_from_file!(gl::VERTEX_SHADER, "../resources/vert.glsl")
}

fn create_frag_shader() -> crate::Result<Shader> {
    shader_from_file!(gl::FRAGMENT_SHADER, "../resources/frag.glsl")
}

fn create_program() -> crate::Result<Program> {
    Program::from_shaders(vec![create_vert_shader()?, create_frag_shader()?])
}

fn create_mesh() -> crate::Result<Mesh> {
    let mut vao = Vao::create();
    vao.bind();

    {
        let vbo = Vbo::create();
        vbo.bind();
        let vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];
        vbo.upload_data(vertices);
        vbo.vertex_attrib(0, 3, gl::FLOAT);
        vbo.unbind();
        vao.add_vbo(vbo);
    }

    {
        let vbo = Vbo::create();
        vbo.bind();
        let vertices: Vec<f32> = vec![
            1.0, 0.0, 0.0, // left
            0.0, 1.0, 0.0, // right
            0.0, 0.0, 1.0, // top
        ];
        vbo.upload_data(vertices);
        vbo.vertex_attrib(1, 3, gl::FLOAT);
        vbo.unbind();
        vao.add_vbo(vbo);
    }

    vao.unbind();

    let program = create_program()?;

    Ok(Mesh::new(vao, program, 3))
}

fn main() -> Result<()> {
    let mut ctx = Voxelar::new();

    ctx.window_hint(WindowHint::ContextVersion(3, 3));
    ctx.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    let (mut window, mut events) = ctx.create_window(800, 600, "Demo", glfw::WindowMode::Windowed);
    window.set_receivable_events(ReceivableEvents::all());

    window.make_current();
    let render_context = window.load_render_context::<GlContext>();
    println!("{}", render_context.get_info()?);

    ctx.set_swap_interval(SwapInterval::Sync(1));

    let mesh = create_mesh()?;

    let program = mesh.program();
    let mut uniform = program.get_uniform("colors")?;
    program.bind();
    uniform.set(Uniforms::Uniform3fv(vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.0, 0.0, 0.5),
    ]));

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        mesh.draw();

        window.swap();
        ctx.poll_events();
        for (_, event) in events.flush() {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut VoxelarWindow, event: glfw::WindowEvent) {
    println!("Event: {:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
