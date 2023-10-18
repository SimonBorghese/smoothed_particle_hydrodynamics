#version 430

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct particle{
    vec2 pos;
    vec2 vel;
    vec2 force;
    vec2 press_force;
    vec2 visc_force;
    float p;
    float rho;
};
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

layout (std430, binding = 2) buffer particles{
    particle Partic[];
};


void main(){
    uint p = gl_GlobalInvocationID.x;
    Partic[p].rho = 0.0;
}

