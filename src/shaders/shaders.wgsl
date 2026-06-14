/*** Compiled shaders from Slang ***/ 

struct _MatrixStorage_float4x4_ColMajorstd140_0
{
    @align(16) data_0 : array<vec4<f32>, i32(4)>,
};

struct CameraUniform_std140_0
{
    @align(16) view_proj_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) view_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) proj_0 : _MatrixStorage_float4x4_ColMajorstd140_0,
    @align(16) cam_pos_0 : vec4<f32>,
    @align(16) time_0 : f32,
    @align(4) _pad1_0 : f32,
    @align(8) _pad2_0 : f32,
    @align(4) _pad3_0 : f32,
};

@binding(0) @group(0) var<uniform> camera_0 : CameraUniform_std140_0;
struct LineVertex_0
{
    @builtin(position) clip_pos_0 : vec4<f32>,
    @location(0) color_0 : vec4<f32>,
    @location(1) age_0 : f32,
};

struct vertexInput_0
{
    @location(0) position_0 : vec3<f32>,
    @location(1) color_1 : vec4<f32>,
    @location(2) age_1 : f32,
};

@vertex
fn vertex_line( _S1 : vertexInput_0) -> LineVertex_0
{
    var output_0 : LineVertex_0;
    output_0.clip_pos_0 = (((vec4<f32>(_S1.position_0, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_0.color_0 = _S1.color_1;
    output_0.age_0 = _S1.age_1;
    return output_0;
}

struct pixelOutput_0
{
    @location(0) output_1 : vec4<f32>,
};

struct pixelInput_0
{
    @location(0) color_2 : vec4<f32>,
    @location(1) age_2 : f32,
};

@fragment
fn fragment_line( _S2 : pixelInput_0, @builtin(position) clip_pos_1 : vec4<f32>) -> pixelOutput_0
{
    var _S3 : pixelOutput_0 = pixelOutput_0( vec4<f32>(vec3<f32>((0.5f + 0.5f * _S2.age_2)) * _S2.color_2.xyz, _S2.color_2.w * (0.15000000596046448f + 0.85000002384185791f * _S2.age_2)) );
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
fn vertex_sphere( _S4 : vertexInput_1) -> SphereVertex_0
{
    var output_2 : SphereVertex_0;
    output_2.clip_pos_2 = (((vec4<f32>(_S4.position_1, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_2.world_pos_0 = _S4.position_1;
    output_2.normal_0 = _S4.normal_1;
    return output_2;
}

struct pixelOutput_1
{
    @location(0) output_3 : vec4<f32>,
};

struct pixelInput_1
{
    @location(0) world_pos_1 : vec3<f32>,
    @location(1) normal_2 : vec3<f32>,
};

@fragment
fn fragment_sphere( _S5 : pixelInput_1, @builtin(position) clip_pos_3 : vec4<f32>) -> pixelOutput_1
{
    var n_0 : vec3<f32> = normalize(_S5.normal_2);
    const SPHERE_RGB_0 : vec3<f32> = vec3<f32>(0.44999998807907104f, 0.69999998807907104f, 1.0f);
    var _S6 : pixelOutput_1 = pixelOutput_1( vec4<f32>(SPHERE_RGB_0 + vec3<f32>(0.08500000089406967f) * SPHERE_RGB_0 + vec3<f32>(max(dot(n_0, normalize(vec3<f32>(1.0f, 2.0f, 1.5f))), 0.0f)) * SPHERE_RGB_0 + vec3<f32>((0.40000000596046448f * pow(1.0f - max(dot(n_0, normalize(camera_0.cam_pos_0.xyz - _S5.world_pos_1)), 0.0f), 5.0f))) * vec3<f32>(0.30000001192092896f, 0.5f, 1.0f), 0.40000000596046448f) );
    return _S6;
}

struct BoxVertex_0
{
    @builtin(position) clip_pos_4 : vec4<f32>,
    @location(0) world_pos_2 : vec3<f32>,
};

struct vertexInput_2
{
    @location(0) position_2 : vec3<f32>,
};

@vertex
fn vertex_box( _S7 : vertexInput_2) -> BoxVertex_0
{
    var output_4 : BoxVertex_0;
    output_4.clip_pos_4 = (((vec4<f32>(_S7.position_2, 1.0f)) * (mat4x4<f32>(camera_0.view_proj_0.data_0[i32(0)][i32(0)], camera_0.view_proj_0.data_0[i32(1)][i32(0)], camera_0.view_proj_0.data_0[i32(2)][i32(0)], camera_0.view_proj_0.data_0[i32(3)][i32(0)], camera_0.view_proj_0.data_0[i32(0)][i32(1)], camera_0.view_proj_0.data_0[i32(1)][i32(1)], camera_0.view_proj_0.data_0[i32(2)][i32(1)], camera_0.view_proj_0.data_0[i32(3)][i32(1)], camera_0.view_proj_0.data_0[i32(0)][i32(2)], camera_0.view_proj_0.data_0[i32(1)][i32(2)], camera_0.view_proj_0.data_0[i32(2)][i32(2)], camera_0.view_proj_0.data_0[i32(3)][i32(2)], camera_0.view_proj_0.data_0[i32(0)][i32(3)], camera_0.view_proj_0.data_0[i32(1)][i32(3)], camera_0.view_proj_0.data_0[i32(2)][i32(3)], camera_0.view_proj_0.data_0[i32(3)][i32(3)]))));
    output_4.world_pos_2 = _S7.position_2;
    return output_4;
}

struct pixelOutput_2
{
    @location(0) output_5 : vec4<f32>,
};

struct pixelInput_2
{
    @location(0) world_pos_3 : vec3<f32>,
};

@fragment
fn fragment_box( _S8 : pixelInput_2, @builtin(position) clip_pos_5 : vec4<f32>) -> pixelOutput_2
{
    var _S9 : pixelOutput_2 = pixelOutput_2( vec4<f32>(1.0f, 1.0f, 1.0f, 0.5f) );
    return _S9;
}

