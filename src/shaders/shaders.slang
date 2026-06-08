// ================================================================================== //
//   Use this Slang shader to compile to WGSL instead of writing WGSL code directly.  //
// ================================================================================== //

// Macros
#define pointwise_mult(a, b) ((a)*(b))   // Just to make it more clear due to usage on 3D vectors

// Camera
struct CameraUniform
{
    float4x4    view_proj;
    float4x4    view;
    float4x4    proj;
    float4      cam_pos;
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

// Fragement shader

// Since this is an object, we can be silly and add a bunch of lighting shenanigans
// Later on, I'll experiment with Ray-tracing
// Right now, Blinn-Phong + ambient lighting + lambertian diffuse + rim lighting
[ForceInline]
float3 blinn_phong_specular(float3 rgb_color, float3 normal_dir, float3 view_dir, 
                            float3 light_color, float3 light_dir, 
                            float3 spec_color, float spec_shininess, float spec_strength) 
{
    float3 half_way = normalize(light_dir + view_dir);
    float specAngle = max(dot(half_way, normal_dir), 0.0);
    float specular = pow(specAngle, spec_shininess);

    float3 modulated_light = pointwise_mult(spec_color, light_color);
    return  (specular*spec_strength) * pointwise_mult(modulated_light, spec_color);
}

[ForceInline] 
float3 apply_ambient_light(float3 rgb_color, float ambient_strength) { return ambient_strength * rgb_color;}

[ForceInline]
float3 lambertian_diffuse(float3 rgb_color, float3 normal_vec, float3 light_dir, float diffuse_strength) {
    float lambertian_coeff = max(dot(normal_vec, light_dir), 0.0);
    return lambertian_coeff * rgb_color;
}

[ForceInline]
float3 rim_lighting(float3 normal_dir, float3 view_dir, 
                    float3 rim_color, float rim_power, float rim_strength) 
{
    float rim_coeff = pow(1.0 - max(dot(normal_dir, view_dir), 0.0), rim_power);
    return (rim_strength*rim_coeff) * rim_color;
}


[shader("fragment")]
float4 sphere_fragement(SphereVertex input) {
    // Base sphere scene data
    float3 n = normalize(input.normal);
    float3 view_dir = normalize(camera.cam_pos.xyz - input.world_pos);
    const float3 SPHERE_RGB = float3(0.45, 0.7, 1.0);
    const float SPHERE_ALPHA = 0.5;

    // Base lighting (hard-coding for now in development)
    const float3 LIGHT_COLOR = float3(1.0); // White light
    const float3 LIGHT_DIRECTION = normalize(float3(1.0, 2.0, 1.5));

    // Blinn-Phong reflection lighting
    const float3 SPECULAR_COLOR = float3(1.0);
    const float SPECULAR_SHININESS = 64.0;
    const float SPECULAR_STRENGTH = 0.8;
    float3 bp_specular_light = blinn_phong_specular(SPHERE_RGB, n, view_dir,
                                                    LIGHT_COLOR, LIGHT_DIRECTION,
                                                    SPECULAR_COLOR, SPECULAR_SHININESS, SPECULAR_STRENGTH);

    // Ambient lighting parameters
    const float AMBIENT_STRENGTH = 0.085;
    float3 ambient_light = apply_ambient_light(SPHERE_RGB, AMBIENT_STRENGTH);

    // Lambertian diffusion parameters
    const float DIFFUSE_STRENGTH = 0.6;
    float3 diffuse = lambertian_diffuse(SPHERE_RGB, n, LIGHT_DIRECTION, DIFFUSE_STRENGTH);

    // Rim-lighting
    const float RIM_STRENGTH = 0.4;
    const float RIM_POWER = 5.0;
    float3 RIM_COLOR = float3(0.3, 0.5, 1.0);
    float3 rim_light = rim_lighting(n, view_dir, RIM_COLOR, RIM_POWER, RIM_STRENGTH);
    

    // Apply additive lighting for tyhe result
    return float4(SPHERE_RGB + ambient_light + diffuse + rim_light, SPHERE_ALPHA);
}

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