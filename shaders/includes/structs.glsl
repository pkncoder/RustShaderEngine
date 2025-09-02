struct Ray {
    vec3 origin;
    vec3 direction;
};

struct Material {
    vec3 color;
};

struct Sphere {
    vec3 origin;
    float radius;
    Material material;    
};

struct HitInfo {
    int hit;
    float dist;
    vec3 hitPos;
    vec3 normal;
    Material material;
};
