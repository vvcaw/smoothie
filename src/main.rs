use smoothie::Smoothie;
use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    smoothie::shake(update);
}

fn update(smoothie: &mut Smoothie) {
    {
        let dom = smoothie.dom();

        dom.insert(0, Box::new(smoothie::Arrow::new(0.4)));

        smoothie.commit();
    }

    let mut duration = smoothie.time_since_start().as_secs_f32();

    loop {
        let dom = smoothie.dom();

        dom.get_mut(&0).unwrap().set_scale((duration % 2f32) / 2f32);

        smoothie.commit();
        duration = smoothie.time_since_start().as_secs_f32();
    }
}
