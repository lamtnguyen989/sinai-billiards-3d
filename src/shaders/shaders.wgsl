struct _MatrixStorage_float4x4_ColMajorstd140_0
{
    @align(16) data_0 : array<vec4<f32>, i32(4)>,
};

struct CameraUniform_std140_0
{
    @align(16) view_proj_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) view_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) proj_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) time_0 : f32,
};

@binding(0) @group(0) var<uniform> camera_0 : CameraUniform_std140_0;
struct LineVertex_0
{
    @builtin(position) clip_pos_0 : vec4<f32>,
    @location(0) color_0 : vec4<f32>,
};

struct vertexInput_0
{
    @location(0) position_0 : vec3<f32>,
    @location(1) color_1 : vec4<f32>,
};

@vertex
fn vertex_line( _S1 : vertexInput_0) -> LineVertex_0
{
    var output_0 : LineVertex_0;
    output_0.clip_pos_0 = (((vec4<f32>(_S1.position_0, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_0.color_0 = _S1.color_1;
    return output_0;
}

struct pixelOutput_0
{
    @location(0) output_1 : vec4<f32>,
};

struct pixelInput_0
{
    @location(0) color_2 : vec4<f32>,
};

@fragment
fn fragment_line( _S2 : pixelInput_0, @builtin(position) clip_pos_1 : vec4<f32>) -> pixelOutput_0
{
    var _S3 : pixelOutput_0 = pixelOutput_0( _S2.color_2 );
    return _S3;
}

struct SphereVertex_0
{
    @builtin(position) clip_pos_2 : vec4<f32>,
    @location(0) world_pos_0 : vec3<f32>,
    @location(1) normal_0 : vec3<f32>,
};

struct vertexInput_1
{
    @location(0) position_1 : vec3<f32>,
    @location(1) normal_1 : vec3<f32>,
};

@vertex
fn sphere_vertex( _S4 : vertexInput_1) -> SphereVertex_0
{
    var output_2 : SphereVertex_0;
    output_2.clip_pos_2 = (((vec4<f32>(_S4.position_1, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_2.world_pos_0 = _S4.position_1;
    output_2.normal_0 = _S4.normal_1;
    return output_2;
}

struct BoxVertex_0
{
    @builtin(position) clip_pos_3 : vec4<f32>,
    @location(0) world_pos_1 : vec3<f32>,
    @location(1) color_3 : vec4<f32>,
};

struct vertexInput_2
{
    @location(0) position_2 : vec3<f32>,
    @location(1) color_4 : vec4<f32>,
};

@vertex
fn box_vertex( _S5 : vertexInput_2) -> BoxVertex_0
{
    var output_3 : BoxVertex_0;
    output_3.clip_pos_3 = (((vec4<f32>(_S5.position_2, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_3.world_pos_1 = _S5.position_2;
    output_3.color_3 = _S5.color_4;
    return output_3;
}

struct pixelOutput_1
{
    @location(0) output_4 : vec4<f32>,
};

struct pixelInput_1
{
    @location(0) world_pos_2 : vec3<f32>,
    @location(1) color_5 : vec4<f32>,
};

@fragment
fn box_color( _S6 : pixelInput_1, @builtin(position) clip_pos_4 : vec4<f32>) -> pixelOutput_1
{
    var _S7 : pixelOutput_1 = pixelOutput_1( _S6.color_5 );
    return _S7;
}

