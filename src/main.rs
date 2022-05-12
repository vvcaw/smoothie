use smoothie::Smoothie;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    smoothie::shake(update);
}

fn update(smoothie: &mut Smoothie) {
    {
        let dom = smoothie.dom();

        dom.insert(0, Box::new(smoothie::Arrow::new(0.8)));

        smoothie.commit();
    }

    let mut duration = smoothie.time_since_start().as_secs_f32();
    while duration < 10.0 {
        let dom = smoothie.dom();

        dom.get_mut(&0).unwrap().set_scale(duration / 10f32);

        smoothie.commit();
        duration = smoothie.time_since_start().as_secs_f32();
    }
}
