HitInfo raySphere(Ray ray, Object sphere) {
    HitInfo hit;
    hit.hit = NO_HIT;
    hit.dist = 9999999999.0;

    vec3 oc = ray.origin.xyz - sphere.location1.xyz;
    float a = dot(ray.direction, ray.direction);
    float b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.location2.x * sphere.location2.x;

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
            hit.normal = normalize(hit.hitPos - sphere.location1.xyz);
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
    vec3 n = m * (ray.origin.xyz - box.location1.xyz);
    vec3 k = abs(m) * vec3(box.location2.x);

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

HitInfo rayTriangle(Ray ray, Object tri) {

    // Initialize hit info
    HitInfo hit;
    hit.hit = NO_HIT;
    hit.dist = 9999999999.0;

    const float EPS = 1e-6;

    // Triangle vertices (stored in Object)
    vec3 v0 = tri.location1.xyz;
    vec3 v1 = tri.location2.xyz;
    vec3 v2 = tri.location3.xyz;

    // Möller–Trumbore intersection (two-sided)
    vec3 edge1 = v1 - v0;
    vec3 edge2 = v2 - v0;

    vec3 pvec = cross(ray.direction, edge2);
    float det = dot(edge1, pvec);

    // Allow both positive and negative determinants
    if (abs(det) < EPS) return hit;

    float invDet = 1.0 / det;
    vec3 tvec = ray.origin - v0;

    float u = dot(tvec, pvec) * invDet;
    if (u < 0.0 || u > 1.0) return hit;

    vec3 qvec = cross(tvec, edge1);
    float v = dot(ray.direction, qvec) * invDet;
    if (v < 0.0 || u + v > 1.0) return hit;

    float t = dot(edge2, qvec) * invDet;

    if (t > EPS) {
        hit.hit = HIT;
        hit.dist = t;
        hit.hitPos = ray.origin + ray.direction * t;

        // Compute face normal (always face against ray)
        vec3 n = normalize(cross(edge1, edge2));
        if (dot(n, ray.direction) > 0.0)
            n = -n;
        hit.normal = n;

        // Assign material from tri.data.w (consistent with your box)
        hit.material = materials[int(tri.data.w)];
    }

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

        else if (currentObject.data[0] == 2.0) {
            currentHit = rayTriangle(ray, currentObject);
        }

        if (currentHit.hit == HIT && currentHit.dist < finalHit.dist) {
            finalHit = currentHit;
        }
    }

    return finalHit;
}
