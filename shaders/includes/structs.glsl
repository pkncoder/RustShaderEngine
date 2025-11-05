struct Ray {
    vec3 origin;
    vec3 direction;
};

struct Material {
    vec4 color;
};

struct Object {
    vec4 location1;
    vec4 location2;
    vec4 location3;
    vec4 location4;
    vec4 data;
};

struct HitInfo {
    int hit;
    float dist;
    vec3 hitPos;
    vec3 normal;
    Material material;
};
