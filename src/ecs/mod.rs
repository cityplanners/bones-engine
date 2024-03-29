#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::{ ControlFlow, EventLoop },
    window::WindowBuilder,
};
pub mod world;
pub mod component;
pub use world::World;

pub type System = fn(&mut World);

pub struct Skeleton {
    world: World,
    init_system: Vec<System>,
    system: Vec<System>,
    event_loop: EventLoop<()>
}

impl Default for Skeleton {
    fn default() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let state = pollster::block_on(crate::engine::State::new(window));

        Self {
            world: World::new(state),
            init_system: Vec::new(),
            system: Vec::new(),
            event_loop,
        }
    }
}

impl Skeleton {
    pub fn new() -> Skeleton {
        Skeleton::default()
    }

    pub fn run(self) {
        pollster::block_on(self._internal_run());
    }

    #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
    async fn _internal_run(mut self) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        // iterate over init systems
        for &system in &self.init_system[..] {
            system(&mut self.world);
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            engine.set_inner_size(PhysicalSize::new(450, 400));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-canvas")?;
                    let canvas = web_sys::Element::from(engine.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let mut last_render_time = instant::Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
        
            // iterate over systems
            for &system in &self.system[..] {
                system(&mut self.world);
            }
            
            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion{ delta, },
                    .. // We're not using device_id currently
                } => if self.world.state.mouse_pressed {
                    self.world.state.camera_controller.process_mouse(delta.0, delta.1)
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.world.state.window().id() => {
                    if !self.world.state.input(event) {
                        match event {
                            #[cfg(not(target_arch="wasm32"))]
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                self.world.state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                self.world.state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.world.state.window().id() => {
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    self.world.state.update(dt);
                    match self.world.state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => self.world.state.resize(self.world.state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.world.state.window().request_redraw();
                }
                _ => {}
            }
        });
    }

    pub fn add_init_system(mut self, system: System) -> Skeleton {
        self.init_system.push(system);
        self
    }

    pub fn add_system(mut self, system: System) -> Skeleton {
        self.system.push(system);
        self
    }
}
