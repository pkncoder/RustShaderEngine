subroutine(shade) vec3 phong(Ray ray, HitInfo hit) {
    vec3 lightPos = vec3(1.5, 2.8, 4.0);
    vec3 lightColor = vec3(1.0);

    vec3 lightDir = normalize(lightPos - hit.hitPos);

    vec3 ambient = ambient.xyz * ambient.w * lightColor * hit.material.color.xyz;

    vec3 diffuse = max(dot(hit.normal, lightDir), 0.0) * lightColor * hit.material.color.xyz;

    vec3 reflectedDir = reflect(-lightDir, hit.normal);
    vec3 specular = pow(max(dot(normalize(ray.origin - hit.hitPos), reflectedDir), 0.0), 16) * lightColor * 1.0;

 
    return (ambient + diffuse + specular);
}
