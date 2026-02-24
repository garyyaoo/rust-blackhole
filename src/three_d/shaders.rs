//! GLSL shader source strings.

pub const GRID_VERT: &str = r#"
    #version 330 core
    layout(location = 0) in vec3 aPos;
    uniform mat4 viewProj;
    void main() {
        gl_Position = viewProj * vec4(aPos, 1.0);
    }
"#;

pub const GRID_FRAG: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(0.5, 0.5, 0.5, 0.7);
    }
"#;

pub const QUAD_VERT: &str = r#"
    #version 330 core
    layout(location = 0) in vec2 aPos;
    layout(location = 1) in vec2 aTexCoord;
    out vec2 vTex;
    void main() {
        gl_Position = vec4(aPos, 0.0, 1.0);
        vTex = aTexCoord;
    }
"#;

/// Schwarzschild null-geodesic ray tracer. y-polar convention.
pub const QUAD_FRAG_GEODESIC: &str = r#"
    #version 330 core
    in  vec2  vTex;
    out vec4  FragColor;

    uniform vec3  camPos;
    uniform vec3  camRight;
    uniform vec3  camUp;
    uniform vec3  camForward;
    uniform float tanHalfFov;
    uniform float aspect;
    uniform float r_s;

    const int   MAX_OBJECTS = 8;
    uniform int  numObjects;
    uniform vec4 objPosRadius[MAX_OBJECTS]; // xyz = position, w = visual radius
    uniform vec4 objColor[MAX_OBJECTS];     // rgb = colour

    const float D_LAMBDA   = 5e9;
    const int   MAX_STEPS  = 3000;
    const float ESCAPE_R   = 1e12;
    const float DISK_INNER = 2.2;
    const float DISK_OUTER = 5.2;
    const float POLE_EPS   = 0.001;  // near polar axis: zero dphi to avoid singularity

    void geodesic_rhs(float r, float theta,
                      float dr, float dtheta, float dphi, float E,
                      out float d2r, out float d2theta, out float d2phi) {
        float f     = 1.0 - r_s / r;
        float dt_dl = E / f;
        float sin_t = max(sin(theta), 1e-6);
        float cos_t = cos(theta);
        d2r     = -(r_s / (2.0*r*r)) * f * dt_dl*dt_dl
                  + (r_s / (2.0*r*r*f)) * dr*dr
                  + r * (dtheta*dtheta + sin_t*sin_t*dphi*dphi);
        d2theta = -2.0*dr*dtheta/r + sin_t*cos_t*dphi*dphi;
        d2phi   = -2.0*dr*dphi/r   - 2.0*(cos_t/sin_t)*dtheta*dphi;
    }

    void main() {
        float u = (vTex.x * 2.0 - 1.0) * aspect * tanHalfFov;
        float v = (vTex.y * 2.0 - 1.0) * tanHalfFov;
        vec3 dir = normalize(u * camRight + v * camUp + camForward);

        float r     = length(camPos);
        float theta = acos(clamp(camPos.y / r, -1.0, 1.0));
        float phi   = atan(camPos.z, camPos.x);

        float sin_theta_raw = sin(theta);
        float sin_t = max(sin_theta_raw, 1e-6);
        float cos_t = cos(theta);
        float sin_p = sin(phi);
        float cos_p = cos(phi);

        float dx = dir.x, dy = dir.y, dz = dir.z;
        float dr     =  sin_t*cos_p*dx + cos_t*dy     + sin_t*sin_p*dz;
        float dtheta = (cos_t*cos_p*dx - sin_t*dy     + cos_t*sin_p*dz) / r;
        float dphi   = (-sin_p*dx                      + cos_p*dz) / (r * sin_t);
        if (abs(sin_theta_raw) < POLE_EPS) dphi = 0.0;

        float f     = 1.0 - r_s / r;
        float dt_dl = sqrt(dr*dr/f + r*r*(dtheta*dtheta + sin_t*sin_t*dphi*dphi));
        float E     = f * dt_dl;

        float disk_r1 = r_s * DISK_INNER;
        float disk_r2 = r_s * DISK_OUTER;
        float prev_y  = camPos.y;

        for (int i = 0; i < MAX_STEPS; i++) {
            if (r <= r_s) {
                FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                return;
            }

            float proximity = clamp((r - r_s) / (5.0 * r_s), 0.0, 1.0);
            float h = D_LAMBDA * (0.05 + 0.95 * proximity);

            float d2r, d2theta, d2phi;
            geodesic_rhs(r, theta, dr, dtheta, dphi, E, d2r, d2theta, d2phi);
            r      += h * dr;
            theta  += h * dtheta;
            phi    += h * dphi;
            dr     += h * d2r;
            dtheta += h * d2theta;
            dphi   += h * d2phi;
            if (abs(sin(theta)) < POLE_EPS) dphi = 0.0;

            float sin_th = sin(theta);
            float cart_x = r * sin_th * cos(phi);
            float cart_y = r * cos(theta);
            float cart_z = r * sin_th * sin(phi);

            if (prev_y * cart_y < 0.0) {
                float xz_r = sqrt(cart_x*cart_x + cart_z*cart_z);
                if (xz_r >= disk_r1 && xz_r <= disk_r2) {
                    float t = (xz_r - disk_r1) / (disk_r2 - disk_r1);
                    FragColor = vec4(1.0, 0.55 + 0.45*t, 0.1*(1.0 - t), 1.0);
                    return;
                }
            }
            prev_y = cart_y;

            // Scene object sphere intersection (headlamp: camera = light source)
            vec3 P = vec3(cart_x, cart_y, cart_z);
            for (int j = 0; j < numObjects; j++) {
                if (distance(P, objPosRadius[j].xyz) <= objPosRadius[j].w) {
                    vec3 N = normalize(P - objPosRadius[j].xyz);
                    vec3 V = normalize(camPos - P);
                    float intensity = 0.1 + 0.9 * max(dot(N, V), 0.0);
                    FragColor = vec4(objColor[j].rgb * intensity, 1.0);
                    return;
                }
            }

            if (r > ESCAPE_R) break;
        }

        FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
"#;
