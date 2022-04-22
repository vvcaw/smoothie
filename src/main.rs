use smoothie::Smoothie;
use std::thread;
use std::time::Duration;

fn main() {
    smoothie::shake(update);
}

fn update(smoothie: &mut Smoothie) {
    /*{
            let dom = smoothie.dom();
            dom.insert(String::from("Hello"), String::from("World!"));
        }

        thread::sleep(Duration::new(2, 0));

        smoothie.commit();

        thread::sleep(Duration::new(5, 0));
    */

    for i in 0..100000 {
        let dom = smoothie.dom();

        dom.insert(String::from("Hello"), String::from("World!"));

        // thread::sleep(Duration::new(1, 0));

        //println!("{:?}", dom);

        smoothie.commit();
    }
}
