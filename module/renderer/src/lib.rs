
/* qqq : move all code to renderer/src and maybe remove submodules?
*/

/* qqq : make bin targets for each platform
is it possible to cross-compile: osx, windows, linux...?
*/

/* qqq : main should be single
*/

/* qqq : esc should terminate application */

/* qqq : is it possible to stop watching if application was terminated? */

/* !!! qqq : seems webgl backend of WebGPU is broken? */

/* qqq : all variables should be move to public config. now template have lots of variables inlined into different files */

#[cfg( target_arch = "wasm32" )]
use web_log::println as println;

pub use wgpu;
pub use winit;
pub use pollster;
pub use bytemuck;

// pub use self::common::*;

pub use std::borrow::Cow;
pub use byte_slice_cast::*;

/* qqq : remove what possible */
use wgpu::
{
  Device,
  Queue,
  RenderPipeline,
  Surface,
  SurfaceConfiguration,
  util::DeviceExt,
};

/* qqq : remove */
use winit::
{
  event::{ Event, WindowEvent },
  event_loop::{ ControlFlow, EventLoop },
  window::Window,
};

pub trait Renderer
{
  fn new() -> Self;

  fn run( &self ) -> ()
  {
    let event_loop = EventLoop::new();
    let window = Window::new( &event_loop ).unwrap();
    pollster::block_on(run( event_loop, window ) );
  }
}

//

#[repr( C )]
#[derive( Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable )]
struct TimeUniformData
{
  time : [ i32; 4 ],
}

impl TimeUniformData
{
  fn new() -> Self
  {
    Self
    {
      time : [ 0, 0, 0, 0 ],
    }
  }
}

// unsafe impl<> ToByteSlice for TimeUniformData
// {
//   fn to_byte_slice< T : AsRef<[ TimeUniformData ]> + ?Sized >( src : &T ) -> &[ u8 ]
//   {
//     let src = src.as_ref();
//     let size = core::mem::size_of::< TimeUniformData >();
//     unsafe
//     {
//       core::slice::from_raw_parts( src.as_ptr() as *const u8, size )
//     }
//   }
// }

// impl AsByteSlice for TimeUniformData
// {
//   fn as_byte_slice( &self ) -> &[ u8 ]
//   {

//   }
// }

//

pub struct Context
{
  config : SurfaceConfiguration,
  surface : Surface,
  device : Device,
  render_pipeline : RenderPipeline,
  queue : Queue,
  time_uniform_data : TimeUniformData,
  time_buffer : wgpu::Buffer,
  time_bind_group_layout : wgpu::BindGroupLayout,
  time_bind_group : wgpu::BindGroup,
}

//

impl Context
{
  async fn new( window: &Window ) -> Context
  {
    let size = window.inner_size();

    // #[cfg(target_arch = "wasm32")]
    // log::debug!( "{:?}", size );
    // qqq : discuss
    println!( "size : {:?}", &size );

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

    /* - uniform - */

    let time_uniform_data = TimeUniformData::new();

    let time_buffer = device.create_buffer_init
    (
      &wgpu::util::BufferInitDescriptor
      {
        label : Some( "Camera Buffer" ),
        contents : bytemuck::cast_slice( &[ time_uniform_data ] ),
        // contents : &[ time_uniform_data ].as_byte_slice(),
        usage : wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      }
    );

    let time_bind_group_layout = device.create_bind_group_layout( &wgpu::BindGroupLayoutDescriptor
    {
      entries :
      &[
        wgpu::BindGroupLayoutEntry
        {
          binding : 0,
          visibility : wgpu::ShaderStages::VERTEX_FRAGMENT,
          ty : wgpu::BindingType::Buffer
          {
            ty : wgpu::BufferBindingType::Uniform,
            has_dynamic_offset : false,
            min_binding_size : None,
          },
          count : None,
        }
      ],
      label: Some( "time_bind_group_layout" ),
    });

    let time_bind_group = device.create_bind_group( &wgpu::BindGroupDescriptor
    {
      layout : &time_bind_group_layout,
      entries :
      &[
        wgpu::BindGroupEntry
        {
          binding : 0,
          resource : time_buffer.as_entire_binding(),
        }
      ],
      label: Some( "time_bind_group" ),
    });

    /* qqq : add uniform time */
    /* - uniform - */

    eprintln!( "Get pipeline layout" );
    let pipeline_layout = device.create_pipeline_layout( &wgpu::PipelineLayoutDescriptor
    {
      label : None,
      bind_group_layouts :
      &[
        &time_bind_group_layout,
      ],
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
      // present_mode : wgpu::PresentMode::Mailbox, /* ! */
      present_mode : wgpu::PresentMode::Fifo,
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

      time_uniform_data,
      time_buffer,
      time_bind_group_layout,
      time_bind_group,

    }
  }
}

//

pub async fn run( event_loop : EventLoop<()>, window : Window )
{

  let mut option_state : Option<Context> = if cfg!( target_os = "android" )
  {
    None
  }
  else
  {
    Some( pollster::block_on( Context::new( &window ) ) )
  };

  event_loop.run( move | event, _, control_flow |
  {
    // Have the closure take ownership of the resources.
    // `event_loop.run` never returns, therefore we must do this to ensure
    // the resources are properly cleaned up.
    // let _ = (&instance, &adapter, &shader, &pipeline_layout);

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    *control_flow = ControlFlow::Poll;

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // *control_flow = ControlFlow::Wait;

    match &mut option_state
    {
      Some( c ) =>
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
            c.config.width = size.width;
            c.config.height = size.height;
            c.surface.configure( &c.device, &c.config );
          }
          Event::MainEventsCleared =>
          {
            /* https://docs.rs/winit/latest/winit/event/enum.Event.html#variant.MainEventsCleared */
            window.request_redraw();
          }
          Event::RedrawRequested( _ ) => /* qqq : Find event that fires every frame */
          {
            let frame = c.surface
            .get_current_texture()
            .expect( "Failed to acquire next swap chain texture" );

            c.time_uniform_data.time[ 0 ] += 1;
            // c.queue.write_buffer( &c.time_buffer, 0, &[ c.time_uniform_data ].as_byte_slice() );
            c.queue.write_buffer( &c.time_buffer, 0, bytemuck::cast_slice(&[c.time_uniform_data]) );
            println!( "time : {}", c.time_uniform_data.time[ 0 ] );

            let view = frame.texture.create_view( &wgpu::TextureViewDescriptor::default() );
            let mut encoder = c.device.create_command_encoder( &wgpu::CommandEncoderDescriptor { label : None } );
            {

              let mut rpass = encoder.begin_render_pass( &wgpu::RenderPassDescriptor
              {
                label : None,
                color_attachments :
                &[
                  wgpu::RenderPassColorAttachment
                  {
                    view : &view,
                    resolve_target : None,
                    ops : wgpu::Operations
                    {
                      load : wgpu::LoadOp::Clear( wgpu::Color::WHITE ),
                      store : true,
                    },
                  }
                ],
                depth_stencil_attachment : None,
              });
              rpass.set_pipeline( &c.render_pipeline );

              // time_uniform_data : TimeUniformData,
              // time_buffer : wgpu::Buffer,
              // time_bind_group_layout : wgpu::BindGroupLayout,

              rpass.set_bind_group( 0, &c.time_bind_group, &[] );
              // rpass.set_bind_group( 1, &c.camera_bind_group, &[] );
              // rpass.set_vertex_buffer( 0, c.vertex_buffer.slice(..) );
              // rpass.set_index_buffer( c.index_buffer.slice(..), wgpu::IndexFormat::Uint16 );
              // rpass.draw_indexed( 0..c.num_indices, 0, 0..1 );
              rpass.draw( 0..3, 0..1 );
            }

            c.queue.submit( Some( encoder.finish() ) );
            frame.present();
          }
          Event::WindowEvent{ event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
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
            std::thread::sleep( std::time::Duration::from_millis( 250 ) );
            option_state = Some( pollster::block_on( Context::new( &window ) ) );
          }
          _ => {}
        }
      }
    }
  });

}
