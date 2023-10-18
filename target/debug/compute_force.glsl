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
    float MASS = 2.5;
    float VISC = 400.0;
    if (ps.x == ps.y){
        return;
    }
    vec2 rij = Partic[ps.y].pos - Partic[ps.x].pos;
    float r = sqrt(pow(rij.x, 2.0) + pow(rij.y, 2.0));

    if (r < kernel_radius){
        Partic[ps.x].press_force = Partic[ps.x].press_force + (normalize(-rij) * mass * (Partic[ps.x].p + Partic[ps.y].p) / (2.0 * Partic[ps.y].rho) * SPIKY_GRAD() * pow((kernel_radius - r), 3.0));
        Partic[ps.x].visc_force = Partic[ps.x].visc_force + ((visc * mass) * (Partic[ps.y].vel - Partic[ps.x].vel) / Partic[ps.y].rho * VISC_LAP() * (kernel_radius - r));
    }

}

