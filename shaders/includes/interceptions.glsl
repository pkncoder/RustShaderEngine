HitInfo raySphere(Ray ray, Sphere sphere) {

    HitInfo hit;
    hit.hit = false;
    hit.dist = 9999999999.0;

    vec3 oc = ray.origin - sphere.origin;
    float a = dot(ray.direction, ray.direction);
    float b = dot(oc, ray.direction);
    float c = dot(oc, oc) - sphere.radius * sphere.radius;

    float discriminant = b * b - a * c;

    if (discriminant > 0.0f) {

        float t = (-b - sqrt(discriminant)) / a;

        if(t < 0.0) {
            t = (-b + sqrt(discriminant)) / a;
        }

        if (t > EPSILON) {
            hit.hit = true;
            hit.dist = t;
            hit.hitPos = ray.origin + ray.direction * t;
            hit.normal = normalize(hit.hitPos - sphere.origin);
        }
    }

    return hit;
}
