mod utils;

use std::collections::HashMap;

use three_d::{
    degrees, vec3, Camera, ClearState, Color, CpuMaterial, CpuMesh, DirectionalLight, FrameOutput,
    Geometry, Gm, InstancedMesh, Instances, Mat4, Mesh, Object, OrbitControl, PhysicalMaterial,
    Rad, Viewport, Window, WindowError, WindowSettings,
};
use wasm_bindgen::prelude::*;
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[non_exhaustive]

pub enum RenderingUserEvent<Q: 'static> {
    InternalCreateWindow(
        usize,
        Box<
            dyn FnOnce(
                &EventLoopWindowTarget<RenderingUserEvent<Q>>,
            ) -> Box<
                dyn FnMut(
                    &winit::event::Event<RenderingUserEvent<Q>>,
                    &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<Q>>,
                    &mut winit::event_loop::ControlFlow,
                ),
            >,
        >,
    ),
    InternalDeleteWindow(usize),
    Other(Q),
}

impl<Q: Clone + 'static> Clone for RenderingUserEvent<Q> {
    fn clone(&self) -> Self {
        match self {
            Self::InternalCreateWindow(_, _) => panic!("can't clone InternalCreateWindow"),
            Self::InternalDeleteWindow(_) => panic!("can't clone InternalDeleteWindow"),
            Self::Other(arg0) => Self::Other(arg0.clone()),
        }
    }
}

#[wasm_bindgen]
pub struct RenderingNever(Rendering<()>);

#[wasm_bindgen]
impl RenderingNever {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Rendering::new())
    }

    #[wasm_bindgen]
    pub fn get_proxy(&self) -> MyEventLoopProxy {
        MyEventLoopProxy(self.0.get_proxy(), 0)
    }

    #[wasm_bindgen]
    pub fn run(self) {
        self.0.run()
    }
}

#[wasm_bindgen]
pub struct MyEventLoopProxy(EventLoopProxy<RenderingUserEvent<()>>, usize);

#[wasm_bindgen]
impl MyEventLoopProxy {
    #[wasm_bindgen]
    pub fn send_event(&self) {
        self.0
            .send_event(RenderingUserEvent::Other(()))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
    }

    #[wasm_bindgen]
    pub fn create_window(&mut self, canvas_id: &str) -> usize {
        let value = create_window(canvas_id);
        let id = self.1;
        self.0
            .send_event(RenderingUserEvent::InternalCreateWindow(id, value))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
        self.1 += 1;
        id
    }

    #[wasm_bindgen]
    pub fn delete_window(&self, id: usize) {
        self.0
            .send_event(RenderingUserEvent::InternalDeleteWindow(id))
            .unwrap_or_else(|_| panic!("Something went horribly wrong!"));
    }
}

pub struct Rendering<Q: 'static> {
    event_loop: EventLoop<RenderingUserEvent<Q>>,
}

impl<Q: 'static> Rendering<Q> {
    pub fn new() -> Self {
        utils::set_panic_hook();

        Self {
            event_loop: EventLoopBuilder::with_user_event().build(),
        }
    }

    pub fn get_proxy(&self) -> EventLoopProxy<RenderingUserEvent<Q>> {
        self.event_loop.create_proxy()
    }

    pub fn run(self) -> ! {
        let mut handlers: HashMap<
            usize,
            Box<
                dyn FnMut(
                    &winit::event::Event<RenderingUserEvent<Q>>,
                    &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<Q>>,
                    &mut winit::event_loop::ControlFlow,
                ),
            >,
        > = HashMap::new();

        self.event_loop.run(move |event, target, control_flow| {
            match event {
                Event::UserEvent(RenderingUserEvent::InternalCreateWindow(id, callback)) => {
                    handlers.insert(id, callback(target));
                }
                Event::UserEvent(RenderingUserEvent::InternalDeleteWindow(id)) => {
                    handlers.remove(&id);
                }
                event => {
                    // TODO FIXME remove our custom type wrapper RenderingUserEvent and then maybe we could use an FnOnce above
                    for handler in handlers.values_mut() {
                        handler(&event, target, control_flow);
                    }
                }
            }
        })
    }
}

pub fn create_window(
    canvas_id: &str,
) -> Box<
    dyn FnOnce(
        &EventLoopWindowTarget<RenderingUserEvent<()>>,
    ) -> Box<
        dyn FnMut(
            &winit::event::Event<RenderingUserEvent<()>>,
            &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<()>>,
            &mut winit::event_loop::ControlFlow,
        ),
    >,
> {
    let websys_window = web_sys::window()
        .ok_or(WindowError::WindowCreation)
        .unwrap();
    let document = websys_window
        .document()
        .ok_or(WindowError::DocumentMissing)
        .unwrap();
    let canvas_element = document
        .get_element_by_id(canvas_id)
        .expect("settings doesn't contain canvas and DOM doesn't have a canvas element either")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|e| WindowError::CanvasConvertFailed(format!("{:?}", e)))
        .unwrap();

    println!("canvas {}", canvas_element.id());

    let callback = Box::new(
        |event_loop: &EventLoopWindowTarget<RenderingUserEvent<()>>| {
            let window = Window::from_event_loop(
                WindowSettings {
                    title: "Instanced Shapes!".to_string(),
                    max_size: Some((1280, 720)),
                    canvas: Some(canvas_element),
                    ..Default::default()
                },
                event_loop,
            )
            .unwrap();
            let context = window.gl();

            let mut camera = Camera::new_perspective(
                window.viewport(),
                vec3(60.00, 50.0, 60.0), // camera position
                vec3(0.0, 0.0, 0.0),     // camera target
                vec3(0.0, 1.0, 0.0),     // camera up
                degrees(45.0),
                0.1,
                1000.0,
            );
            let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

            let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
            let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

            // Container for non instanced meshes.
            let mut non_instanced_meshes = Vec::new();

            // Instanced mesh object, initialise with empty instances.
            let mut instanced_mesh = Gm::new(
                InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
                PhysicalMaterial::new(
                    &context,
                    &CpuMaterial {
                        albedo: Color {
                            r: 128,
                            g: 128,
                            b: 128,
                            a: 255,
                        },
                        ..Default::default()
                    },
                ),
            );
            instanced_mesh.set_animation(|time| Mat4::from_angle_x(Rad(time)));

            // Initial properties of the example, 2 cubes per side and non instanced.
            let side_count = 4;
            let is_instanced = true;

            let inner_callback: Box<
                dyn FnMut(
                    &winit::event::Event<RenderingUserEvent<()>>,
                    &winit::event_loop::EventLoopWindowTarget<RenderingUserEvent<()>>,
                    &mut winit::event_loop::ControlFlow,
                ),
            > = Box::new(window.get_render_loop::<RenderingUserEvent<()>, _>(
                move |mut frame_input| {
                    let viewport = Viewport {
                        x: 0,
                        y: 0,
                        width: frame_input.viewport.width,
                        height: frame_input.viewport.height,
                    };
                    camera.set_viewport(viewport);

                    // Camera control must be after the gui update.
                    control.handle_events(&mut camera, &mut frame_input.events);

                    // Ensure we have the correct number of cubes, does no work if already correctly sized.
                    let count = side_count * side_count * side_count;
                    if non_instanced_meshes.len() != count {
                        non_instanced_meshes.clear();
                        for i in 0..count {
                            let mut gm = Gm::new(
                                Mesh::new(&context, &CpuMesh::cube()),
                                PhysicalMaterial::new(
                                    &context,
                                    &CpuMaterial {
                                        albedo: Color {
                                            r: 128,
                                            g: 128,
                                            b: 128,
                                            a: 255,
                                        },
                                        ..Default::default()
                                    },
                                ),
                            );
                            let x = (i % side_count) as f32;
                            let y = ((i as f32 / side_count as f32).floor() as usize % side_count)
                                as f32;
                            let z = (i as f32 / side_count.pow(2) as f32).floor();
                            gm.set_transformation(Mat4::from_translation(
                                3.0 * vec3(x, y, z)
                                    - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                            ));
                            gm.set_animation(|time| Mat4::from_angle_x(Rad(time)));
                            non_instanced_meshes.push(gm);
                        }
                    }

                    if instanced_mesh.instance_count() != count as u32 {
                        instanced_mesh.set_instances(&Instances {
                            transformations: (0..count)
                                .map(|i| {
                                    let x = (i % side_count) as f32;
                                    let y = ((i as f32 / side_count as f32).floor() as usize
                                        % side_count)
                                        as f32;
                                    let z = (i as f32 / side_count.pow(2) as f32).floor();
                                    Mat4::from_translation(
                                        3.0 * vec3(x, y, z)
                                            - 1.5 * (side_count as f32) * vec3(1.0, 1.0, 1.0),
                                    )
                                })
                                .collect(),
                            ..Default::default()
                        });
                    }

                    // Always update the transforms for both the normal cubes as well as the instanced versions.
                    // This shows that the difference in frame rate is not because of updating the transforms
                    // and shows that the performance difference is not related to how we update the cubes.
                    let time = (frame_input.accumulated_time * 0.001) as f32;
                    instanced_mesh.animate(time);
                    non_instanced_meshes
                        .iter_mut()
                        .for_each(|m| m.animate(time));

                    // Then, based on whether or not we render the instanced cubes, collect the renderable
                    // objects.
                    let render_objects: Vec<&dyn Object> = if is_instanced {
                        instanced_mesh.into_iter().collect()
                    } else {
                        non_instanced_meshes
                            .iter()
                            .map(|x| x as &dyn Object)
                            .collect()
                    };

                    frame_input
                        .screen()
                        .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                        .render(&camera, render_objects, &[&light0, &light1]);

                    FrameOutput::default()
                },
            ));
            inner_callback
        },
    );
    callback
}
