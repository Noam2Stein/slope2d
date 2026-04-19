struct Triangle {
    vertex_1: vec2f,
    vertex_2: vec2f,
    vertex_3: vec2f,
    color: vec4f,
};

struct Fragment {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
};

@vertex
fn vs_main(
    @location(0) vertex_1: vec2f,
    @location(1) vertex_2: vec2f,
    @location(2) vertex_3: vec2f,
    @location(3) color: vec4f,
    @builtin(vertex_index) vertex_index: u32,
) -> Fragment {
    var vertex: vec2f;
    switch (vertex_index) {
        case 0u: {
            vertex = vertex_1;
        }
        case 1u: {
            vertex = vertex_2;
        }
        default: {
            vertex = vertex_3;
        }
    }

    var fragment: Fragment;
    fragment.position = vec4f(vertex, 0.0, 1.0);
    fragment.color = color;
    return fragment;
}

@fragment
fn fs_main(in: Fragment) -> @location(0) vec4f {
    return in.color;
}
