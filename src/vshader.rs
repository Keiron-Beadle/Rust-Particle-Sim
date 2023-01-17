pub fn get() -> &'static str{
    return r#"#version 140

    in vec3 position;
    in vec3 colour;
    uniform mat4 world;
    uniform mat4 view;
    uniform mat4 projection;
    out vec3 sampleColour;

    void main() {
        gl_Position = projection * view * world * vec4(position, 1.0);
        sampleColour = colour;
    }
"#;
}
