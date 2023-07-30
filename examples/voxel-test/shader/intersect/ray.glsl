#ifndef ray_glsl
#define ray_glsl

struct Ray {
    vec3 origin;
    vec3 direction;
};

vec3 computeRayIntersection(Ray ray, float distance) {
    return ray.origin + ray.direction * distance;
}

#endif
