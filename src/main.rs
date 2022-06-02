use smoothie::{animate, Easing};
use std::f32::consts::PI;

fn main() {
    let mut smoothie = smoothie::shake();

    let mut arrow = smoothie.arrow();
    arrow.scale = 0.4;

    // TODO: create a macro like `arrow!()` that allows to create a arrow with optional arguments

    animate! {
        smoothie;
        arrow,x => 2.5; // x and y are not yet updated in render_state
        arrow,y => 2.5;
    };

    animate! {
        smoothie;
        easing = Easing::Linear;
        duration = 4.0;
        arrow,angle => 2.0 * PI;
    }

    animate! {
        smoothie;
        arrow,scale => 0.3 + arrow.scale;
    }

    smoothie.serve();
}
