#![allow( dead_code )]

/* qqq : make bin targets for each platform
is it possible to cross-compile: osx, windows, linux...?
*/

/* qqq : esc should terminate application */

/* qqq : is it possible to stop watching if application was terminated? */

/* qqq : seems webgl backend of WebGPU is broken? aaa:repaired */

/* qqq : all variables should be move to public config. now template have lots of variables inlined into different files */

#[cfg( feature = "wee_alloc" )]
#[global_allocator]
static ALLOC : wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg( target_arch = "wasm32" )]
use winit::platform::web::WindowExtWebSys;

#[cfg( target_arch = "wasm32" )]
#[allow( unused_imports )]
use web_log::{ println as println, eprintln as eprintln };

pub use wgpu;
pub use winit;
pub use pollster;

pub use std::borrow::Cow;
pub use byte_slice_cast::*;

/* qqq : remove what possible aaa:done*/
pub use wgpu::util::DeviceExt;

pub trait Renderer
{
  fn new() -> Self;

  fn run( &self ) -> ()
  {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new( &event_loop ).unwrap();
    pollster::block_on(run( event_loop, window ) );
  }
}

//

#[repr( C )]
#[derive( Debug, Copy, Clone )]
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

unsafe fn any_as_u8_slice< T : Sized >( p : &T ) -> &[ u8 ]
{
  ::std::slice::from_raw_parts(( p as *const T ) as *const u8,::std::mem::size_of::< T >() )
}

//

pub struct Context
{
  config : wgpu::SurfaceConfiguration,
  surface : wgpu::Surface,
  device : wgpu::Device,
  render_pipeline : wgpu::RenderPipeline,
  queue : wgpu::Queue,
  time_uniform_data : TimeUniformData,
  time_buffer : wgpu::Buffer,
  time_bind_group_layout : wgpu::BindGroupLayout,
  time_bind_group : wgpu::BindGroup,
}

//

impl Context
{
  async fn new( window: &winit::window::Window ) -> Context
  {
    let size = window.inner_size();

    #[cfg( not( target_os = "android" ) )]
    let instance = wgpu::Instance::new( wgpu::Backends::all() );
    #[cfg( target_os = "android" )]
    let instance = wgpu::Instance::new( wgpu::Backends::GL );

    let surface = unsafe { instance.create_surface( window ) };

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
    eprintln!( "Using custom LIMITS!, qqq: investigate me" );
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
        // contents : &[ time_uniform_data ].as_byte_slice(),
        contents : unsafe{ any_as_u8_slice( &time_uniform_data ) },
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

    /* - uniform - */

    let pipeline_layout = device.create_pipeline_layout( &wgpu::PipelineLayoutDescriptor
    {
      label : None,
      bind_group_layouts :
      &[
        &time_bind_group_layout,
      ],
      push_constant_ranges : &[],
    });

    let swapchain_format = surface.get_preferred_format( &adapter ).unwrap();

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

    surface.configure( &device, &config );

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
// Event handlers
//

pub fn window_resize_handle( c : &mut Context, size : winit::dpi::PhysicalSize< u32 > )
{
    // Reconfigure the surface with the new size
    // println!( "Resized to: {:#?}", size );
    c.config.width = size.width;
    c.config.height = size.height;
    c.surface.configure( &c.device, &c.config );
}

//

pub fn window_before_redraw_handle( window : &winit::window::Window)
{
   /*
    https://docs.rs/winit/latest/winit/event/enum.Event.html#variant.MainEventsCleared
    qqq : Find event that fires every frame aaa:done
    Emitted when all of the event loop’s input events have been processed and redraw processing is about to begin.
  */
  #[cfg( target_arch = "wasm32" )]
  {
    let canvas = window.canvas();
    let ( canvas_width, canvs_height ) = (canvas.client_width(), canvas.client_height());

    let js_window = web_sys::window().unwrap();
    let window_width = js_window.inner_width().unwrap().as_f64().unwrap() as i32;
    let window_height = js_window.inner_height().unwrap().as_f64().unwrap() as i32;

    if canvas_width != window_width || canvs_height != window_height
    {
      window.set_inner_size( winit::dpi::PhysicalSize::new( window_width, window_height ) );
    }
  }

  window.request_redraw();
}

//

pub fn window_redraw_handle( c: &mut Context )
{
  let frame = c.surface
  .get_current_texture()
  .expect( "Failed to acquire next swap chain texture" );

  c.time_uniform_data.time[ 0 ] += 1;

  // c.queue.write_buffer( &c.time_buffer, 0, &[ c.time_uniform_data ].as_byte_slice() );
  c.queue.write_buffer( &c.time_buffer, 0, unsafe{ any_as_u8_slice( &c.time_uniform_data ) } );
  // println!( "time : {}", c.time_uniform_data.time[ 0 ] );

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

//

pub fn window_close_request_handle( control_flow : &mut winit::event_loop::ControlFlow )
{
  *control_flow = winit::event_loop::ControlFlow::Exit
}

//

pub fn window_key_released_handle( virtual_code: winit::event::VirtualKeyCode, control_flow : &mut winit::event_loop::ControlFlow )
{
  // println!( "Pressed: {:#?}", virtual_code );
  match virtual_code
  {
    winit::event::VirtualKeyCode::Escape => window_close_request_handle( control_flow ),
    _ => {}
  }
}

//
// Run function
//

pub async fn run( event_loop : winit::event_loop::EventLoop<()>, window : winit::window::Window )
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
    // *control_flow = winit::event_loop::ControlFlow::Poll;
    *control_flow = winit::event_loop::ControlFlow::Poll;

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // *control_flow = winit::event_loop::ControlFlow::Wait;

    match &mut option_state
    {
      Some( c ) =>
      {
        match event
        {
          winit::event::Event::WindowEvent { event : winit::event::WindowEvent::Resized( size ), .. } => window_resize_handle( c, size ),
          winit::event::Event::MainEventsCleared => window_before_redraw_handle( &window ),
          winit::event::Event::RedrawRequested( _ ) => window_redraw_handle( c ),
          winit::event::Event::WindowEvent { event : winit::event::WindowEvent::CloseRequested, .. } => window_close_request_handle( control_flow ),
          winit::event::Event::WindowEvent { event : winit::event::WindowEvent::KeyboardInput { input : winit::event::KeyboardInput { virtual_keycode: Some( virtual_code ), state: winit::event::ElementState::Released, .. }, .. }, .. } => window_key_released_handle( virtual_code, control_flow ),
          _ => {}
        };
      }
      _ =>
      {
        match event
        {
          winit::event::Event::Resumed =>
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