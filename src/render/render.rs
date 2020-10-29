use cgmath::Matrix4;
use glfw::{Action, Context as _, Key, WindowEvent};
use log::*;
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::tess::Mode;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use crate::render::camera;
use crate::render::cube;

fn main() {
    let dim = WindowDim::Windowed {
        width: 512,
        height: 512,
    };
    let surface = GlfwSurface::new_gl33("Minecator", WindowOpt::default().set_dim(dim));

    match surface {
        Ok(surface) => {
            info!("Graphics surface created");
            main_loop(surface);
        }

        Err(e) => {
            error!("Cannot create graphics surface:\n{}", e);
            std::process::exit(1);
        }
    }
}

fn main_loop(mut surface: GlfwSurface) {
    let (width, height) = surface.window.get_framebuffer_size();
    let mut camera_state = camera::CameraState::new(width, height);

    let mut cube_program = surface
        .new_shader_program::<cube::Semantics, (), camera::CameraShaderInterface>()
        .from_strings(cube::VS, None, None, cube::FS)
        .expect("Cube shader creation")
        .ignore_warnings();

    let (cube_vertices, cube_indices) = cube::cube(0.5);
    let cube = surface
        .new_tess()
        .set_vertices(&cube_vertices[..])
        .set_indices(&cube_indices[..])
        .set_mode(Mode::TriangleStrip)
        .set_primitive_restart_index(cube::VertexIndex::max_value())
        .build()
        .unwrap();

    let mut back_buffer = surface.back_buffer().unwrap();
    let mut resize = false;

    'app: loop {
        surface.window.glfw.poll_events();
        for (_, event) in surface.events_rx.try_iter() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }

                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }

                WindowEvent::Key(Key::A, _, Action::Release, _)
                | WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                    camera_state.strafe_left();
                }

                WindowEvent::Key(Key::D, _, Action::Release, _)
                | WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                    camera_state.strafe_right();
                }

                WindowEvent::Key(Key::W, _, Action::Release, _)
                | WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                    camera_state.strafe_forward();
                }

                WindowEvent::Key(Key::S, _, Action::Release, _)
                | WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                    camera_state.strafe_backward();
                }

                WindowEvent::Key(Key::Q, _, Action::Release, _)
                | WindowEvent::Key(Key::Q, _, Action::Repeat, _) => {
                    camera_state.strafe_up();
                }

                WindowEvent::Key(Key::E, _, Action::Release, _)
                | WindowEvent::Key(Key::E, _, Action::Repeat, _) => {
                    camera_state.strafe_down();
                }

                WindowEvent::Scroll(_, scroll) => {
                    camera_state.change_fov(scroll);
                }

                _ => (),
            }
        }

        if resize {
            back_buffer = surface.back_buffer().unwrap();

            let [width, height] = back_buffer.size();
            camera_state.recalc_size(width, height);
            resize = false;
        }

        let projection = camera_state.projection.into();
        let view = Matrix4::from(camera_state.cam_view).into();

        let render = surface
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shd_gate| {
                    shd_gate.shade(&mut cube_program, |mut iface, unis, mut rdr_gate| {
                        iface.set(&unis.projection, projection);
                        iface.set(&unis.view, view);
                        iface.set(&unis.aspect_ratio, camera_state.aspect_ratio);

                        rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&cube)
                        })
                    })
                },
            )
            .assume();

        if render.is_ok() {
            surface.window.swap_buffers();
        } else {
            break 'app;
        }
    }
}
