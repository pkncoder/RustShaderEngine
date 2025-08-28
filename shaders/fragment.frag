#version 330 core

in vec2 fragPosition;
out vec4 fragColor;

uniform vec2 iResolution;

#include <structs.glsl>
#include <defines.glsl>
#include <interceptions.glsl>

void main() {
    vec2 uv = (fragPosition.xy * vec2(iResolution.x / iResolution.y, 1.0));

    Ray ray = Ray(
        vec3(0.0),
        normalize(vec3(uv, 1.0))
    );

    Sphere sphere = Sphere(
        vec3(0.0, 0.0, 5.0),
        1.0,
        vec3(0.7, 0.0, 0.1)
    );

    HitInfo hit = raySphere(ray, sphere);

    vec3 lightPos = vec3(1.5, 2.8, 4.0);
    vec3 lightDir = normalize(lightPos - hit.hitPos);

    vec3 color = vec3(0.0);

    if (hit.hit) {
      color = sphere.color * max(dot(hit.normal, lightDir), 0.0);
    }

    fragColor = vec4(color, 1.0);
}
