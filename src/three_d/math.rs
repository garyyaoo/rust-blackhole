//! Column-major 4×4 and vec3 helpers (GLM-style).

pub fn perspective(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fov_y_rad * 0.5).tan();
    let nf = 1.0 / (near - far);
    let mut m = [0.0f32; 16];
    m[0] = f / aspect;
    m[5] = f;
    m[10] = (far + near) * nf;
    m[11] = -1.0;
    m[14] = 2.0 * far * near * nf;
    m
}

pub fn look_at(eye: [f32; 3], center: [f32; 3]) -> [f32; 16] {
    let up = [0.0f32, 1.0, 0.0];
    let f = vec3_norm(vec3_sub(center, eye));
    let s = vec3_norm(vec3_cross(f, up));
    let u = vec3_cross(s, f);

    let mut m = [0.0f32; 16];
    m[0] = s[0];
    m[4] = s[1];
    m[8] = s[2];
    m[12] = -vec3_dot(s, eye);
    m[1] = u[0];
    m[5] = u[1];
    m[9] = u[2];
    m[13] = -vec3_dot(u, eye);
    m[2] = -f[0];
    m[6] = -f[1];
    m[10] = -f[2];
    m[14] = vec3_dot(f, eye);
    m[15] = 1.0;
    m
}

pub fn mat4_mul(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut c = [0.0f32; 16];
    for col in 0..4 {
        for row in 0..4 {
            for k in 0..4 {
                c[col * 4 + row] += a[k * 4 + row] * b[col * 4 + k];
            }
        }
    }
    c
}

fn vec3_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec3_norm(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn vec3_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn vec3_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Returns (right, up, forward) unit vectors for a camera at `pos` looking at origin.
pub fn camera_basis(pos: [f32; 3]) -> ([f32; 3], [f32; 3], [f32; 3]) {
    let fwd = vec3_norm([-pos[0], -pos[1], -pos[2]]);
    let right = vec3_norm(vec3_cross(fwd, [0.0, 1.0, 0.0]));
    let up = vec3_cross(right, fwd);
    (right, up, fwd)
}
