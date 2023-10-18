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

layout (std430, binding = 2) buffer particles{
    particle Partic[];
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


void main(){
    uint p = gl_GlobalInvocationID.x;
    float deltaTime = 0.0007f;
    float EPS = eps;
    float BOUND_DAMPING = bound_damping;
    Partic[p].vel += (deltaTime * (Partic[p].force / Partic[p].rho));
    Partic[p].pos += (Partic[p].vel * deltaTime);
    particle par = Partic[p];


    if (par.pos.x - EPS < 0.0){
        Partic[p].vel.x *= BOUND_DAMPING;
        Partic[p].pos.x = EPS;
    }
    if (par.pos.x + EPS > 1600.0){
        Partic[p].vel.x *= BOUND_DAMPING;
        Partic[p].pos.x = 1600 - EPS;
    }


    if (par.pos.y - EPS < 0.0){
        Partic[p].vel.y *= BOUND_DAMPING;
        Partic[p].pos.y = EPS;
    }
    if (par.pos.y + EPS > 900.0){
        Partic[p].vel.y *= BOUND_DAMPING;
        Partic[p].pos.y = 900 - EPS;
    }

}

