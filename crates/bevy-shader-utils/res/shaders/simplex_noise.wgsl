#define_import_path bevy_shader_utils::simplex_noise

const PERM = array<u32, 256>(
    151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,
    140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,
    247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,
    57,177,33,88,237,149,56,87,174,20,125,136,171,168,68,175,
    74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,
    60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,
    65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,
    200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,
    52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,
    207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,
    119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,
    129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,
    218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,
    81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,
    184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,
    222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
);

const GRAD2 = array<vec2<f32>, 8>(
    vec2(1.0, 1.0), vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(-1.0, -1.0),
    vec2(1.0, 0.0), vec2(-1.0, 0.0), vec2(0.0, 1.0), vec2(0.0, -1.0)
);

const GRAD3 = array<vec3<f32>, 16>(
    vec3(1.0, 1.0, 0.0), vec3(-1.0, 1.0, 0.0), vec3(1.0, -1.0, 0.0), vec3(-1.0, -1.0, 0.0),
    vec3(1.0, 0.0, 1.0), vec3(-1.0, 0.0, 1.0), vec3(1.0, 0.0, -1.0), vec3(-1.0, 0.0, -1.0),
    vec3(0.0, 1.0, 1.0), vec3(0.0, -1.0, 1.0), vec3(0.0, 1.0, -1.0), vec3(0.0, -1.0, -1.0),
    vec3(1.0, 1.0, 0.0), vec3(-1.0, 1.0, 0.0), vec3(0.0, -1.0, 1.0), vec3(0.0, -1.0, -1.0)
);

fn hash(i: u32) -> u32 {
    return PERM[i & 255u];
}

fn noise2d(pos: vec2<f32>) -> f32 {
    var n0: f32 = 0.0;
    var n1: f32 = 0.0;
    var n2: f32 = 0.0;

    let F2: f32 = 0.366025404; // (sqrt(3) - 1) / 2
    let G2: f32 = 0.211324865; // (3 - sqrt(3)) / 6

    let s = (pos.x + pos.y) * F2;
    let xs = pos.x + s;
    let ys = pos.y + s;
    let i = floor(xs);
    let j = floor(ys);

    let t = (i + j) * G2;
    let X0 = i - t;
    let Y0 = j - t;
    let x0 = pos.x - X0;
    let y0 = pos.y - Y0;

    var i1: u32;
    var j1: u32;
    if (x0 > y0) {
        i1 = 1u;
        j1 = 0u;
    } else {
        i1 = 0u;
        j1 = 1u;
    }

    let x1 = x0 - f32(i1) + G2;
    let y1 = y0 - f32(j1) + G2;
    let x2 = x0 - 1.0 + 2.0 * G2;
    let y2 = y0 - 1.0 + 2.0 * G2;

    let t0 = 0.5 - x0 * x0 - y0 * y0;
    if (t0 > 0.0) {
        let gi0 = hash(u32(i) + hash(u32(j))) & 7u;
        n0 = t0 * t0 * t0 * t0 * dot(GRAD2[gi0], vec2(x0, y0));
    }

    let t1 = 0.5 - x1 * x1 - y1 * y1;
    if (t1 > 0.0) {
        let gi1 = hash(u32(i) + i1 + hash(u32(j) + j1)) & 7u;
        n1 = t1 * t1 * t1 * t1 * dot(GRAD2[gi1], vec2(x1, y1));
    }

    let t2 = 0.5 - x2 * x2 - y2 * y2;
    if (t2 > 0.0) {
        let gi2 = hash(u32(i) + 1u + hash(u32(j) + 1u)) & 7u;
        n2 = t2 * t2 * t2 * t2 * dot(GRAD2[gi2], vec2(x2, y2));
    }

    return 70.0 * (n0 + n1 + n2);
}

fn noise3d(pos: vec3<f32>) -> f32 {
    var n0: f32 = 0.0;
    var n1: f32 = 0.0;
    var n2: f32 = 0.0;
    var n3: f32 = 0.0;

    let F3: f32 = 1.0 / 3.0;
    let G3: f32 = 1.0 / 6.0;

    let s = (pos.x + pos.y + pos.z) * F3;
    let xs = pos.x + s;
    let ys = pos.y + s;
    let zs = pos.z + s;
    let i = floor(xs);
    let j = floor(ys);
    let k = floor(zs);

    let t = (i + j + k) * G3;
    let X0 = i - t;
    let Y0 = j - t;
    let Z0 = k - t;
    let x0 = pos.x - X0;
    let y0 = pos.y - Y0;
    let z0 = pos.z - Z0;

    var i1: u32;
    var j1: u32;
    var k1: u32;
    var i2: u32;
    var j2: u32;
    var k2: u32;

    if (x0 >= y0) {
        if (y0 >= z0) {
            i1 = 1u; j1 = 0u; k1 = 0u; i2 = 1u; j2 = 1u; k2 = 0u;
        } else if (x0 >= z0) {
            i1 = 1u; j1 = 0u; k1 = 0u; i2 = 1u; j2 = 0u; k2 = 1u;
        } else {
            i1 = 0u; j1 = 0u; k1 = 1u; i2 = 1u; j2 = 0u; k2 = 1u;
        }
    } else {
        if (y0 < z0) {
            i1 = 0u; j1 = 0u; k1 = 1u; i2 = 0u; j2 = 1u; k2 = 1u;
        } else if (x0 < z0) {
            i1 = 0u; j1 = 1u; k1 = 0u; i2 = 0u; j2 = 1u; k2 = 1u;
        } else {
            i1 = 0u; j1 = 1u; k1 = 0u; i2 = 1u; j2 = 1u; k2 = 0u;
        }
    }

    let x1 = x0 - f32(i1) + G3;
    let y1 = y0 - f32(j1) + G3;
    let z1 = z0 - f32(k1) + G3;
    let x2 = x0 - f32(i2) + 2.0 * G3;
    let y2 = y0 - f32(j2) + 2.0 * G3;
    let z2 = z0 - f32(k2) + 2.0 * G3;
    let x3 = x0 - 1.0 + 3.0 * G3;
    let y3 = y0 - 1.0 + 3.0 * G3;
    let z3 = z0 - 1.0 + 3.0 * G3;

    let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
    if (t0 > 0.0) {
        let gi0 = hash(u32(i) + hash(u32(j) + hash(u32(k)))) & 15u;
        n0 = t0 * t0 * t0 * t0 * dot(GRAD3[gi0], vec3(x0, y0, z0));
    }

    let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
    if (t1 > 0.0) {
        let gi1 = hash(u32(i) + i1 + hash(u32(j) + j1 + hash(u32(k) + k1))) & 15u;
        n1 = t1 * t1 * t1 * t1 * dot(GRAD3[gi1], vec3(x1, y1, z1));
    }

    let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
    if (t2 > 0.0) {
        let gi2 = hash(u32(i) + i2 + hash(u32(j) + j2 + hash(u32(k) + k2))) & 15u;
        n2 = t2 * t2 * t2 * t2 * dot(GRAD3[gi2], vec3(x2, y2, z2));
    }

    let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
    if (t3 > 0.0) {
        let gi3 = hash(u32(i) + 1u + hash(u32(j) + 1u + hash(u32(k) + 1u))) & 15u;
        n3 = t3 * t3 * t3 * t3 * dot(GRAD3[gi3], vec3(x3, y3, z3));
    }

    return 32.0 * (n0 + n1 + n2 + n3);
}
