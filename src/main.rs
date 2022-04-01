fn main() {
    let app = smoothie::App::new();
    pollster::block_on(app.run());
}
