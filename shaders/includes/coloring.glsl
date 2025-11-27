vec3 color(Ray ray, HitInfo hit) {
    if (hit.hit == NO_HIT) {
        return getSkyColor(ray);
    }

    if (hit.hit == IN_SHADOW) {
        vec3 lightColor = vec3(1.0);

        return ambient.xyz * ambient.w * lightColor * hit.material.color.xyz;
    }

    else if (hit.hit == HIT) {
        return modelColoring(ray, hit);
    }

    return vec3(1.0, 0.0, 1.0);
}
