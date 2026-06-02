/***
*   Use Slang shader to compile to WGSL instead of writing WGSL code directly.
***/

// Camera
struct CameraUniform
{
    float4x4    view_proj;
    float4x4    view;
    float4x4    proj;
    float       time;
};

ConstantBuffer<CameraUniform> camera;

/***
*   Trajectory line graphics data and shaders
***/

// Input data
struct LineInput
{
    float3 position : POSITION;
    float4 color    : COLOR;
};

// Processed vertex data for trajectory line
struct LineVertex
{
    float4 clip_pos : SV_Position;
    float4 color    : COLOR;
};

// Vertex shader
[shader("vertex")]
LineVertex vertex_line(LineInput input) {
    LineVertex output;
    output.clip_pos = mul(camera.view_proj, float4(input.position, 1.0));
    output.color = input.color;

    return output;
}

// Fragment shader
[shader("fragment")]
float4 fragment_line(LineVertex line_vertex): SV_Target {
    return line_vertex.color;
}

/***
*   Sphere graphics data and shaders
***/

// Input data
struct SphereInput
{
    float3 position : POSITION;
    float3 normal   : NORMAL;
};

// Vertex shader output data
struct SphereVertex
{
    float4 clip_pos : SV_Position;
    float3 world_pos: TEXCOORD0;
    float3 normal   : TEXCOORD1;
};

// Vertex shader
[shader("vertex")]
SphereVertex sphere_vertex(SphereInput input) {
    SphereVertex output;
    output.clip_pos = mul(camera.view_proj, float4(input.position, 1.0));
    output.world_pos = input.position;
    output.normal = input.normal;
    return output;
}

/*
// Fragement shader
[shader("fragment")]
float4 sphere_fragement(SphereVertexOut sphere_vertex) {
    // TODO: Cool colors go here
}
*/

/***
*   Bounding box graphics data and shaders
***/

// Input data for box
struct BoxInput
{
    float3 position : POSITION;
    float4 color    : COLOR;
};

// Vertex shader output data for box
struct BoxVertex
{
    float4 clip_pos : SV_Position;
    float3 world_pos: TEXCOORD0;
    float4 color    : COLOR;
};

// Vertex shader
[shader("vertex")]
BoxVertex box_vertex(BoxInput input) {
    BoxVertex output;
    output.clip_pos = mul(camera.view_proj, float4(input.position, 1.0));
    output.world_pos = input.position;
    output.color = input.color;
    return output;
}

[shader("fragment")]
float4 box_color(BoxVertex input) {
    return input.color;
}