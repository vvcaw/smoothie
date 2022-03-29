fn main() {
    let renderer = smoothie::Renderer::new();
    pollster::block_on(renderer.run());
}
