
[[block]]
struct TimeUniform
{
  time : vec4<i32>;
};

[[group( 0 ), binding( 0 )]]
var< uniform > u1 : TimeUniform;

struct VertexInput
{
  [[builtin( vertex_index )]] vertex_index : u32;
};

struct VertexOutput
{
  [[ builtin( position ) ]] position : vec4< f32 >;
};

struct FragmentOutput
{
  [[location( 0 )]] color0 : vec4< f32 >;
};

[[stage( vertex )]]
fn vs_main( in : VertexInput ) -> VertexOutput
{
  let x = f32( i32( in.vertex_index ) - 1);
  let y = f32( i32( in.vertex_index & 1u ) * 2 - 1);
  var out : VertexOutput;
  out.position = vec4< f32 >( x, y, 0.0, 1.0 );
  return out;
}

[[stage( fragment )]]
fn fs_main( in : VertexOutput ) -> FragmentOutput
{
  var out : FragmentOutput;
  out.color0 = vec4< f32 >( 1.0, in.position[ 1 ] / 1000.0, f32( u1.time.x ) / 1000., 1.0 );
  return out;
}
