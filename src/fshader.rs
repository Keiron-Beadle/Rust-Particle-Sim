pub fn get() -> &'static str{
    return r#"
    #version 140
    in vec3 sampleColour;
    out vec4 color;

    void main() {
        color = vec4(sampleColour, 1.0);
    }
"#;
}
