#version 410 core

#extension GL_ARB_shader_subroutine : require

in vec2 fragPosition;
out vec4 fragColor;

uniform vec2 iResolution;
uniform vec4 ambient;
uniform float time;

#include <structs.glsl>
#include <subroutines.glsl>
#include <defines.glsl>

#include <rotations.glsl>

// layout(std140) uniform ObjectBlock {
//     Object objects[10];
//     float objects_length;
// };

uniform samplerBuffer objects;
uniform float objects_length;

Object fetchObject(int objectIndex) {
    int base_texel_index = objectIndex * 5;

    Object obj;
    // Use texelFetch to get data at an integer index from the 1D texture.
    obj.location1 = texelFetch(objects, base_texel_index + 0);
    // obj.location1.y = texelFetch(objects, base_texel_index + 1).w;
    // obj.location1.z = texelFetch(objects, base_texel_index + 2).w;

    obj.location2 = texelFetch(objects, base_texel_index + 1);
    // obj.location2.y = texelFetch(objects, base_texel_index + 5).w;
    // obj.location2.z = texelFetch(objects, base_texel_index + 6).w;

    obj.location3 = texelFetch(objects, base_texel_index + 2);
    // obj.location3.y = texelFetch(objects, base_texel_index + 9).w;
    // obj.location3.z = texelFetch(objects, base_texel_index + 10).w;

    obj.location4 = texelFetch(objects, base_texel_index + 3);
    // obj.location4.y = texelFetch(objects, base_texel_index + 13).w;
    // obj.location4.z = texelFetch(objects, base_texel_index + 14).w;

    obj.data = texelFetch(objects, base_texel_index + 4);

    return obj;
}

layout(std140) uniform MaterialBlock {
    Material materials[10];
    float materials_length;
};

#include <interceptions.glsl>

#include <rayTracing.glsl>

#include <sky.glsl>

#include <phong.glsl>
#include <lambertion.glsl>

#include <coloring.glsl>

#include <srgb.glsl>
#include <toneMapping.glsl>

void main() {
    // fragColor = vec4(objects_length / 10.0, 0.0, 0.0, 0.0);
    // return;

    int max_idx = textureSize(objects);
    // if (true) {
    //     fragColor = vec4(1, 0, 0, 1); // draw red if reading past end
    //     return;
    // }

    vec2 uv = (fragPosition.xy * vec2(iResolution.x / iResolution.y, 1.0));
    float cameraDist = 1.0f / tan(FOV * 0.5f * PI / 180.0f);

    Ray ray = Ray(
            -vec3(0.0, 0.0, 8.0) * rotate(time / 100.0) + vec3(0.0, 1.0, 0.0),
            normalize(vec3(uv, cameraDist)) * rotateX(-7.0) * rotate(time / 100.0)
        );

    HitInfo hit = rayTrace(ray);

    vec3 col = color(ray, hit);

    col = LinearToSRGB(ACESFilm(col));

    fragColor = vec4(col, 1.0);
}
