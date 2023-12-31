#version 430

layout (local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout (rgba32f, binding = 0) uniform image2D img_output;


void main(){
    vec4 color = vec4(0.0, 0.0, 0.0, 1.0);

    ivec2 coords = ivec2(gl_GlobalInvocationID.xy);

    imageStore(img_output, coords, color);
    //}

}