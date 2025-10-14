HitInfo raySphere(Ray ray, Sphere sphere) {

    HitInfo hit;
    hit.hit = NO_HIT;
    hit.dist = 9999999999.0;

    vec3 oc = ray.origin.xyz - sphere.origin.xyz;
    float a = dot(ray.direction, ray.direction);
    float b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.origin.w * sphere.origin.w;

    float discriminant = b * b - a * c;

    if (discriminant > 0.0f) {

        float t = (-b - sqrt(discriminant)) / a;

        if(t < 0.0) {
            t = (-b + sqrt(discriminant)) / a;
        }

        if (t > EPSILON) {
            hit.hit = HIT;
            hit.dist = t;
            hit.hitPos = ray.origin + ray.direction * t;
            hit.normal = normalize(hit.hitPos - sphere.origin.xyz);
            hit.material = materials[int(sphere.data.w)];
        }
    }

    return hit;
}

HitInfo rayScene(Ray ray) {
    HitInfo finalHit;
    finalHit.hit = NO_HIT;
    finalHit.dist = 9999999.0;

    HitInfo currentHit;

    for (int i = 0; i < spheres_length; i++) {
      Sphere currentSphere = spheres[i];

      currentHit = raySphere(ray, currentSphere);

      if (currentHit.hit == HIT && currentHit.dist < finalHit.dist) {
        finalHit = currentHit;
      }
    }

    return finalHit;
}
