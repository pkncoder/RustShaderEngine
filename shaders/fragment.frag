#version 330 core

#extension GL_ARB_shader_subroutine : require

in vec2 fragPosition;
out vec4 fragColor;

uniform vec2 iResolution;
uniform vec4 ambient;



#include <structs.glsl>
#include <subroutines.glsl>
#include <defines.glsl>

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

    Ray ray = Ray(
        vec3(0.0),
        normalize(vec3(uv, 1.0))
    );

    Sphere sphere = Sphere(
        vec3(0.0, 0.0, 5.0),
        1.0,
        Material(
            vec3(1.0, 0.0, 0.0)
        )
    );

    HitInfo hit = rayTrace(ray, sphere);

    vec3 col = color(ray, hit); 

    col = LinearToSRGB(ACESFilm(col));

    fragColor = vec4(col, 1.0);
}
