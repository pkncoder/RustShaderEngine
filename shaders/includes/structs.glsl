struct Ray {
    vec3 origin;
    vec3 direction;
};

struct Sphere {
    vec3 origin;
    float radius;
    vec3 color;
};

struct HitInfo {
  bool hit;
  float dist;
  vec3 hitPos;
  vec3 normal;
};

