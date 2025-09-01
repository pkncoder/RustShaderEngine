vec3 simpleShading(Ray ray, HitInfo hit) {
    vec3 lightPos = vec3(1.5, 2.8, 4.0);
    vec3 lightColor = vec3(1.0);

    vec3 lightDir = normalize(lightPos - hit.hitPos);

    vec3 ambient = ambient.xyz * ambient.w * lightColor * hit.material.color;
    vec3 diffuse = max(dot(hit.normal, lightDir), 0.0) * lightColor * hit.material.color;

    return (ambient + diffuse);
}
