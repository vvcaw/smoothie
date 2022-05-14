use smoothie::{animate, Arrow, Smoothie};

fn main() {
    let smoothie = smoothie::shake();

    //let arrow = smoothie.arrow();

    #[derive(Debug)]
    struct Test {
        x: f32,
        y: f32,
        id: usize,
    }

    let arrow = Test {
        x: 5.0,
        y: 5.0,
        id: 0,
    };

    animate! {
        arrow,x => 10.0
        arrow,y => 9.0;
        with
        duration = 4.0;
        easing = "Linear"
    }

    smoothie.serve();
}
