struct Ray {
    vec3 origin;
    vec3 direction;
};

struct Material {
    vec4 color;
};

struct Sphere {
    vec4 origin;
    vec4 data;
};

struct HitInfo {
    int hit;
    float dist;
    vec3 hitPos;
    vec3 normal;
    Material material;
};
