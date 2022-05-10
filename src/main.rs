use smoothie::Smoothie;

fn main() {
    smoothie::shake(update);
}

fn update(smoothie: &mut Smoothie) {
    let dom = smoothie.dom();

    dom.insert(0, Box::new(smoothie::Arrow::new(0.8)));

    smoothie.commit();

    for i in 0..10000 {}
}
