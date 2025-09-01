vec3 getSkyColor(Ray ray) {
    return mix(vec3(0.8, 0.4, 0.0), vec3(0.1, 0.4, 0.5), sin(dot(vec3(0.0, 1.0, 0.0), ray.direction) + 0.5));
}
