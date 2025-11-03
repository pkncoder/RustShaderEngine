HitInfo raySphere(Ray ray, Object sphere) {
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

        if (t < 0.0) {
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

HitInfo rayBox(Ray ray, Object box) {

    // Inital hit info
    HitInfo hit;
    hit.hit = NO_HIT;
    hit.dist = 9999999999.0;

    vec3 m = 1.0 / ray.direction;
    vec3 n = m * (ray.origin.xyz - box.origin.xyz);
    vec3 k = abs(m) * vec3(box.origin.w);

    vec3 t1 = -n - k;
    vec3 t2 = -n + k;

    float tN = max(max(t1.x, t1.y), t1.z);
    float tF = min(min(t2.x, t2.y), t2.z);

    if (tN > tF || tF < 0.) return hit;

    float t = tN < 0.0 ? tF : tN;

    if (t < 0.0) {
        return hit;
    }

    hit.hit = HIT;
    hit.dist = t;
    hit.hitPos = ray.origin + ray.direction * t;
    hit.normal = normalize(-sign(ray.direction) * step(t1.yzx, t1.xyz) * step(t1.zxy, t1.xyz));
    hit.material = materials[int(box.data.w)];

    return hit;
}

HitInfo rayScene(Ray ray) {
    HitInfo finalHit;
    finalHit.hit = NO_HIT;
    finalHit.dist = 9999999.0;

    HitInfo currentHit;

    for (int i = 0; i < objects_length; i++) {
        Object currentObject = objects[i];

        if (currentObject.data[0] == 0.0) {
            currentHit = raySphere(ray, currentObject);
        }

        else if (currentObject.data[0] == 1.0) {
            currentHit = rayBox(ray, currentObject);
        }

        if (currentHit.hit == HIT && currentHit.dist < finalHit.dist) {
            finalHit = currentHit;
        }
    }

    return finalHit;
}
