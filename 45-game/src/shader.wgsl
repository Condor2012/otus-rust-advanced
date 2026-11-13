
struct Vertex {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

@vertex
fn vs_main(@location(0) position: vec2f, @location(1) color: vec3f) -> Vertex {
    return Vertex(vec4f(position, 0.0, 1.0), color);
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4f {
    return vec4f(vertex.color, 1.0);
}
