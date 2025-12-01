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

// Objects
uniform samplerBuffer objects;
uniform float objects_length;

Object fetchObject(int objectIndex) {
    // Each index needs to skip 5 slots (each index takes 5 slots)
    int base_texel_index = objectIndex * 5;

    // Init a zeroed object
    Object obj;

    // Location 1
    obj.location1 = texelFetch(objects, base_texel_index + 0);

    // Location 2
    obj.location2 = texelFetch(objects, base_texel_index + 1);

    // Location 3
    obj.location3 = texelFetch(objects, base_texel_index + 2);

    // Location 4
    obj.location4 = texelFetch(objects, base_texel_index + 3);

    // Data block
    obj.data = texelFetch(objects, base_texel_index + 4);

    // Return the final object
    return obj;
}

// Materials
uniform samplerBuffer materials;
uniform float material_length;

Material fetchMaterial(int materialIndex) {
    // Each index needs to skip 1 slots (each index takes 1 slots)
    int base_texel_index = materialIndex * 1;

    // Init a zeroed material
    Material mat;

    // Set the main albedo for the material
    mat.color = texelFetch(materials, base_texel_index + 0);

    // Return the final material
    return mat;
}

#include <interceptions.glsl>

#include <rayTracing.glsl>

#include <sky.glsl>

#include <phong.glsl>
#include <lambertion.glsl>

#include <coloring.glsl>

#include <srgb.glsl>
#include <toneMapping.glsl>

void main() {
    vec2 uv = (fragPosition.xy * vec2(iResolution.x / iResolution.y, 1.0));
    uv.y *= -1.0; // TODO : Anyway to not with the texture?
    float cameraDist = 1.0f / tan(FOV * 0.5f * PI / 180.0f);

    Ray ray = Ray(
            -vec3(0.0, 0.0, 8.0) * rotate(time / 100.0) + vec3(0.0, 1.0, 0.0),
            normalize(vec3(uv, cameraDist)) * rotateX(-7.0) * rotate(time / 100.0)
        );

    // Ray ray = Ray(
    //         vec3(0.0, 0.0, -8.0),
    //         normalize(vec3(uv, cameraDist))
    //     );

    // TODO: Create a way to set light positions
    HitInfo hit = rayTrace(ray);

    vec3 col = color(ray, hit);

    col = LinearToSRGB(ACESFilm(col));

    fragColor = vec4(col, 1.0);
}
