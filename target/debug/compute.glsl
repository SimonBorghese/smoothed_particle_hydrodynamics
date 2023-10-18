#version 430

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout (rgba32f, binding = 0) uniform image2D img_output;

struct particle{
    vec2 pos;
    vec2 vel;
    vec2 force;
    vec2 press_force;
    vec2 visc_force;
    float p;
    float rho;
};

layout (std430, binding = 2) buffer particles{
    particle Partic[];
};

/*
pub struct PARAMS{
    pub PIXEL_SIZE: i32,
    pub REST_DENS: f32,
    pub GAS_CONST: f32,
    pub KERNEL_RADIUS: f32,
    pub KR_SQ: f32,
    pub MASS: f32,
    pub VISC: f32,
    pub EPS: f32,
    pub BOUND_DAMPING: f32

}

*/
layout (std140, binding = 4) uniform params{
    int pixel_size;
    float rest_dens;
    float gas_const;
    float kernel_radius;
    float KR_SQ;
    float mass;
    float visc;
    float eps;
    float bound_damping;
};


void main(){
    vec4 color = vec4(0.0, 0.0, 1.0, 1.0);

    ivec2 coords = ivec2(gl_GlobalInvocationID.xy);
    vec2 fcoords = vec2(coords);
    ivec2 dims = imageSize(img_output);



    //for (int p = 0; p < 400; p++){

    uint p = gl_GlobalInvocationID.x;
    /*
        if (abs(Partic[p].pos.x - coords.x) < 8){
            if (abs(Partic[p].pos.y - coords.y) < 8){
                color = vec4(0.0, 0.0, 1.0, 1.0);
                imageStore(img_output, coords, color);
            }
        }
        */
    /*
    vec2 particlePos = Partic[p].pos.xy;
    float dist =  distance(fcoords, particlePos);
    //colorOut[coords.x][coords.y] += 4.0 / dist;
    if (8.0 >= dist){
    //    colorOut[coords.y][coords.x] += 1.0;
        color = vec4(0.0, 0.0, 1.0, 0.0);
        imageStore(img_output, coords, color);
    }
    */
    ivec2 pos = ivec2(Partic[p].pos);
    for (int x = -(pixel_size/2); x < (pixel_size/2); x++){
        for (int y = -(pixel_size/2); y < (pixel_size/2); y++){
            imageStore(img_output, ivec2(Partic[p].pos) + ivec2(x,y), color);
        }
    }

    //imageStore(img_output, coords, color);
    //}

}