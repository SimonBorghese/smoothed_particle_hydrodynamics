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

/*
fn SPIKY_GRAD() -> f32{
    -10.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(5.0))
}
*/
/*
fn VISC_LAP() -> f32{
    40.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(5.0))
}
*/
float SPIKY_GRAD(){
    return -10.0 / (3.14 * pow(kernel_radius, 5.0));
}
float VISC_LAP(){
    return 40.0 / (3.14 * pow(kernel_radius, 5.0));
}

/*
fn POLY6() -> f32{
    4.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(8.0))
}

*/
float POLY6(){
    return 4.0 / (3.14 * pow(kernel_radius, 8.0));
}
void main(){
    /*
    let rij = particles[pj].position - particles[pi].position;
            let r = glm::sqrt(glm::pow(rij.x, 2.0) + glm::pow(rij.y, 2.0));

            if r < KERNEL_RADIUS{
                pressure_force = pressure_force + (glm::normalize(rij.neg()) * MASS * (particles[pi].pressure + particles[pj].pressure) / (2.0 * particles[pj].rho) * SPIKY_GRAD() * (KERNEL_RADIUS - r).powf(3.0));

                viscosity_force = viscosity_force + (glm::to_vec2(VISC * MASS) * (particles[pj].velocity - particles[pi].velocity) / particles[pj].rho * VISC_LAP() * (KERNEL_RADIUS - r));

            }
    */
    ivec2 ps = ivec2(gl_GlobalInvocationID.xy);

    vec2 rij = Partic[ps.y].pos - Partic[ps.x].pos;
    float r2 = (pow(rij.x, 2.0) + pow(rij.y, 2.0));
    if (r2 < KR_SQ){
        Partic[ps.x].rho += mass * POLY6() * pow(KR_SQ - r2, 3.0);
    }

}

