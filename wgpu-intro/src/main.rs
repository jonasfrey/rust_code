use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
 };
 
 fn main() {
     env_logger::init(); // Necessary for logging within WGPU
     let event_loop = EventLoop::new(); // Loop provided by winit for handling window events
 

     let window = WindowBuilder::new().build(&event_loop).unwrap();

     let instance = wgpu::Instance::new(wgpu::Backends::all());
     let surface = unsafe { instance.create_surface(&window) };
     let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
         power_preference: wgpu::PowerPreference::default(),
         compatible_surface: Some(&surface),
         force_fallback_adapter: false,
     }))
     .unwrap();
 
     let (device, queue) = pollster::block_on(adapter.request_device(
         &wgpu::DeviceDescriptor {
             label: None,
             features: wgpu::Features::empty(),
             limits: wgpu::Limits::default(),
         },
         None, // Trace path
     ))
     .unwrap();
 
     let size = window.inner_size();
     surface.configure(&device, &wgpu::SurfaceConfiguration {
         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
         format: surface.get_preferred_format(&adapter).unwrap(),
         width: size.width,
         height: size.height,
         present_mode: wgpu::PresentMode::Fifo,
     });
     
     let mut blue_value = 0.0; // New
let mut blue_inc = 0.0; // Newc
let mut red_inc = 0.0; // New
     let mut n_i = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        
        match event {
            Event::RedrawRequested(_) => {
                let output = surface.get_current_texture().unwrap();
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
     
                {
                    n_i+=1;

                    let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: (1.0/(n_i % 10) as f64), // Pick any color you want here
                                    g: 0.9,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                }
     
                // submit will accept anything that implements IntoIter
                queue.submit(std::iter::once(encoder.finish()));
                output.present();

                // New
                blue_value += (red_inc as f64) * 0.001;
                if blue_value > 1.0 {
                    blue_inc = -1.0;
                    blue_value = 1.0;
                } else if blue_value < 0.0 {
                    blue_inc = 1.0;
                    blue_value = 0.0;
                }
              },
            // New
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == window.id() => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    *control_flow = ControlFlow::Exit
                }
            }
            _ => (),

            
        }
    });

 }