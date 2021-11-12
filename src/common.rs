pub use std::borrow::Cow;
pub use wgpu::
{ 
  Device, 
  Queue, 
  RenderPipeline, 
  Surface, 
  SurfaceConfiguration 
};
pub use winit::
{
  event::{ Event, WindowEvent },
  event_loop::{ ControlFlow, EventLoop },
  window::Window,
};

//

pub struct State 
{
  config : SurfaceConfiguration,
  surface : Surface,
  device : Device,
  render_pipeline : RenderPipeline,
  queue : Queue,
}

impl State 
{
  async fn new( window: &Window ) -> State 
  {
    let size = window.inner_size();

    eprintln!( "Get instance" );
    #[cfg( not( target_os = "android" ) )]
    let instance = wgpu::Instance::new( wgpu::Backends::all() );
    #[cfg( target_os = "android" )]
    let instance = wgpu::Instance::new( wgpu::Backends::GL );

    eprintln!( "Get surface" );
    let surface = unsafe { instance.create_surface( window ) };

    eprintln!( "Get adapter" );
    let adapter_options = wgpu::RequestAdapterOptions 
    {
      power_preference : wgpu::PowerPreference::default(),
      force_fallback_adapter : false,
      // Request an adapter which can render to our surface
      compatible_surface : Some( &surface ),
    };
    let adapter = instance.request_adapter( &adapter_options )
    .await
    .expect( "Failed to find an appropriate adapter" );

    // wgpu::Limits::default().using_resolution(adapter.limits());
    eprintln!( "using my custom LIMITS!" );
    // eprintln!(
    //     "wgpu::Limits::downlevel_webgl2_defaults()
    // .using_resolution(adapter.limits())"
    // );
    // limits: wgpu::Limits::downlevel_webgl2_defaults()
    //     .using_resolution(adapter.limits()),
    let limits = wgpu::Limits 
     {
      max_texture_dimension_1d : 2048,
      max_texture_dimension_2d : 2048,
      max_texture_dimension_3d : 256,
      max_texture_array_layers : 256,
      max_bind_groups : 4,
      max_dynamic_uniform_buffers_per_pipeline_layout : 8,
      // max_dynamic_storage_buffers_per_pipeline_layout: 4,
      max_sampled_textures_per_shader_stage : 16,
      max_samplers_per_shader_stage : 16,
      // max_storage_buffers_per_shader_stage: 4,
      // max_storage_textures_per_shader_stage: 4,
      max_uniform_buffers_per_shader_stage : 12,
      max_uniform_buffer_binding_size : 16384,
      // max_storage_buffer_binding_size: 128 << 20,
      max_vertex_buffers : 8,
      max_vertex_attributes : 16,
      // max_vertex_buffer_array_stride: 2048,
      max_push_constant_size : 0,
      min_uniform_buffer_offset_alignment : 256,
      min_storage_buffer_offset_alignment : 256,
      // These?
      max_storage_buffers_per_shader_stage : 0,
      max_storage_textures_per_shader_stage : 0,
      max_dynamic_storage_buffers_per_pipeline_layout : 0,
      max_storage_buffer_binding_size : 0,
      max_vertex_buffer_array_stride : 255,
    };

    eprintln!( "Get device and queue" );
    // Create the logical device and command queue
    let ( device, queue ) = adapter.request_device( &wgpu::DeviceDescriptor 
    {
      label : None,
      features : wgpu::Features::empty(),
      // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
      limits,
    }, None )
    .await
    .expect( "Failed to create device" );

    eprintln!( "Get shader" );
    // Load the shaders from disk
    let shader = device.create_shader_module( &wgpu::ShaderModuleDescriptor 
    {
      label : None,
      source : wgpu::ShaderSource::Wgsl( Cow::Borrowed( include_str!( "shader.wgsl" ) ) ),
    });

    eprintln!( "Get pipeline layout" );
    let pipeline_layout = device.create_pipeline_layout( &wgpu::PipelineLayoutDescriptor 
    {
      label : None,
      bind_group_layouts : &[],
      push_constant_ranges : &[],
    });

    eprintln!( "Get swap chain format" );
    let swapchain_format = surface.get_preferred_format( &adapter ).unwrap();

    eprintln!( "Get render pipeline" );
    let render_pipeline = device.create_render_pipeline( &wgpu::RenderPipelineDescriptor 
    {
      label : None,
      layout : Some( &pipeline_layout ),
      vertex : wgpu::VertexState 
      {
        module : &shader,
        entry_point : "vs_main",
        buffers : &[],
      },
      fragment : Some( wgpu::FragmentState 
      {
        module : &shader,
        entry_point : "fs_main",
        targets : &[ swapchain_format.into() ],
      }),
      primitive : wgpu::PrimitiveState::default(),
      depth_stencil : None,
      multisample : wgpu::MultisampleState::default(),
    });

    let config = wgpu::SurfaceConfiguration 
    {
      usage : wgpu::TextureUsages::RENDER_ATTACHMENT,
      format : swapchain_format,
      width : size.width,
      height : size.height,
      present_mode : wgpu::PresentMode::Mailbox,
    };

    eprintln!( "Surface configure!" );
    surface.configure( &device, &config );

    eprintln!( "Setup DONE!" );
    Self 
    {
      config,
      surface,
      device,
      render_pipeline,
      queue,
    }
  }
}

pub async fn run( event_loop : EventLoop<()>, window : Window ) 
{
  let mut option_state : Option<State> = if cfg!( target_os = "android" ) 
  {
    None
  } 
  else 
  {
    Some( pollster::block_on(State::new( &window ) ) )
  };

  event_loop.run(move |event, _, control_flow| 
  {
    // Have the closure take ownership of the resources.
    // `event_loop.run` never returns, therefore we must do this to ensure
    // the resources are properly cleaned up.
    // let _ = (&instance, &adapter, &shader, &pipeline_layout);

    *control_flow = ControlFlow::Wait;

    match &mut option_state 
    {
      Some( s) => 
      {
        match event 
        {
          Event::WindowEvent 
          {
            event: WindowEvent::Resized( size ),
            ..
          } => 
          {
            // Reconfigure the surface with the new size
            s.config.width = size.width;
            s.config.height = size.height;
            s.surface.configure( &s.device, &s.config );
          }
          Event::RedrawRequested( _ ) => 
          {
            let frame = s
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture" );

            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default() );
            let mut encoder = s.device.create_command_encoder( &wgpu::CommandEncoderDescriptor { label: None } );
            {
              let mut rpass = encoder.begin_render_pass( &wgpu::RenderPassDescriptor 
              {
                label : None,
                color_attachments : &[wgpu::RenderPassColorAttachment 
                {
                  view : &view,
                  resolve_target : None,
                  ops : wgpu::Operations 
                  {
                    load : wgpu::LoadOp::Clear( wgpu::Color::GREEN ),
                    store : true,
                  },
                }],
                depth_stencil_attachment: None,
              });
              rpass.set_pipeline( &s.render_pipeline );
              rpass.draw( 0..3, 0..1 );
            }

            s.queue.submit(Some( encoder.finish() ) );
            frame.present();
          }
          Event::WindowEvent 
          {
            event: WindowEvent::CloseRequested,
            ..
          } => *control_flow = ControlFlow::Exit,
          _ => {}
        };
      }
      _ => 
      {
        match event 
        {
          Event::Resumed => 
          {
            // log::info!("App resumed");
            std::thread::sleep(std::time::Duration::from_millis( 250 ) );
            option_state = Some( pollster::block_on(State::new( &window ) ) );
          }
          _ => {}
        }
      }
    }
  });
}
