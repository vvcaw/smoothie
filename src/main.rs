fn main() {
    UserRenderer::new().run();
}

struct CustomRenderState {
    count: i32,
}

struct UserRenderer {
    state: CustomRenderState,
}

impl UserRenderer {
    pub fn new() -> Self {
        Self {
            state: CustomRenderState { count: 0 },
        }
    }

    // Pass self by value here
    pub fn run(self) {
        let mut app = smoothie::App::new(
            self.state,
            Box::new(|state: &mut CustomRenderState, dom: &mut Vec<String>| {
                //println!("Count: {:?}", state.count);
                //println!("DOM: {:#?}", dom);
                state.count += 1;
            }),
        );

        pollster::block_on(app.run());
    }
}
