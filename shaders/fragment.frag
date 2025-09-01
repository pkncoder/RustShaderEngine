#version 330 core

in vec2 fragPosition;
out vec4 fragColor;

uniform vec2 iResolution;
uniform vec4 ambient;

#include <structs.glsl>
#include <defines.glsl>

#include <interceptions.glsl>

#include <srgb.glsl>
#include <toneMapping.glsl>

vec3 getSkyColor(Ray ray) {
    return mix(vec3(0.8, 0.4, 0.0), vec3(0.1, 0.4, 0.5), sin(dot(vec3(0.0, 1.0, 0.0), ray.direction) + 0.5));
}

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
    vec3 lightColor = vec3(1.0);
  
    vec3 lightDir = normalize(lightPos - hit.hitPos);
  
    vec3 ambient = ambient.xyz * ambient.w * lightColor * sphere.color;

    vec3 diffuse = max(dot(hit.normal, lightDir), 0.0) * lightColor * sphere.color;
    
    vec3 reflectedDir = reflect(-lightDir, hit.normal);
    vec3 specular = pow(max(dot(normalize(ray.origin - hit.hitPos), reflectedDir), 0.0), 32.0) * lightColor * 1.0;

    vec3 col = getSkyColor(ray);

    if (hit.hit) {
      col = (diffuse + specular + ambient);
    }

    vec3 color = LinearToSRGB(ACESFilm(col));

    fragColor = vec4(color, 1.0);
}
