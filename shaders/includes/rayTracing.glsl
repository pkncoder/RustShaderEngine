HitInfo rayTrace(Ray ray) {
    HitInfo sceneHit = rayScene(ray);

    if (sceneHit.hit == NO_HIT) {
        return sceneHit;
    }

    vec3 lightPos = vec3(3.0, 6.0, -3.0);

    Ray shadowRay = Ray(
            sceneHit.hitPos + sceneHit.normal * EPSILON,
            normalize(lightPos - sceneHit.hitPos)
        );
    Ray cpy = shadowRay;

    HitInfo shadowRayHit = rayScene(shadowRay);

    float maxDist = dot(cpy.origin.xyz - lightPos, cpy.origin.xyz - lightPos);
    if (shadowRayHit.hit == HIT && (shadowRayHit.dist * shadowRayHit.dist < maxDist)) { // Shadow hit
        sceneHit.hit = IN_SHADOW;
    }

    return sceneHit;
}
