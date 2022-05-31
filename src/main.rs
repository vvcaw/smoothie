use smoothie::animate;

fn main() {
    let mut smoothie = smoothie::shake();

    let mut arrow = smoothie.arrow();

    animate! {
        smoothie;
        arrow,x => 2.5; // x and y are not yet updated in render_state
        arrow,y => 2.5;
        arrow,scale => 0.5;
    };

    animate! {
        smoothie;
        arrow,scale => 0.8;
    }

    smoothie.serve();
}
