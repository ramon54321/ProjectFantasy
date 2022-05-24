#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec4 outColor;

#define MAX_STEPS 100
#define MAX_DISTANCE 100.0
#define SURFACE_DISTANCE 0.01

float getDist(vec3 point)
{
    vec4 sphere = vec4(0, 1, 2, 0.5);
    
    float sphereDist = length(point - sphere.xyz) - sphere.w;
    float planeDist = point.y;
    
    float dist = min(sphereDist, planeDist);
    return dist;
}

float rayMarch(vec3 rayOrigin, vec3 rayDirection)
{
    float distanceOrigin = 0.0;
    
    for (int i = 0; i < MAX_STEPS; i++)
    {
        vec3 currentMarchingPoint = rayOrigin + rayDirection * distanceOrigin;
        float distanceScene = getDist(currentMarchingPoint);
        distanceOrigin += distanceScene;
        if (distanceOrigin > MAX_DISTANCE || distanceScene < SURFACE_DISTANCE) break;
    }
    
    return distanceOrigin;
}

vec3 getNormal(vec3 point)
{
    float pointDistance = getDist(point);
    vec2 sampleOffset = vec2(0.01, 0.0);
    vec3 normal = pointDistance - vec3(
        getDist(point - sampleOffset.xyy),
        getDist(point - sampleOffset.yxy),
        getDist(point - sampleOffset.yyx)
    );
    return normalize(normal);
}

vec3 getLight(vec3 point)
{
    vec3 lightPosition = vec3(2.5, 5.0, -1.0);
    vec3 directionToLight = normalize(lightPosition - point);
    vec3 surfaceNormal = getNormal(point);
    vec3 diffuseLight = vec3(dot(surfaceNormal, directionToLight));

    float distanceToLight = length(lightPosition - point);
    float distanceToHitTowardsLight = rayMarch(point + directionToLight * 0.04, directionToLight);

    if (distanceToHitTowardsLight < distanceToLight) {
	diffuseLight *= 0.1;
    }

    vec3 ambientLight = vec3(0.15, 0.05, 0.1) * 1.0;

    return ambientLight + diffuseLight;
}

void main()
{
    vec2 uv = vec2(position.x * (1280.0 / 720.0), position.y);
    vec3 rayOrigin = vec3(0, 1, 0);
    vec3 rayDirection = normalize(vec3(uv.x, uv.y, 1));
    
    float hitDistance = rayMarch(rayOrigin, rayDirection);
    vec3 hitPoint = rayOrigin + rayDirection * hitDistance;
    
    //float diffuseLight = getNormal(hitPoint);
    
    //vec3 color = vec3(diffuseLight);
    vec3 light = getLight(hitPoint);
    vec3 color = light; 

    outColor = vec4(color, 1.0);
}
