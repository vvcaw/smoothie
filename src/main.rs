use std::thread;
use std::time::Duration;

fn main() {
    let mut smoothie = smoothie::shake();

    {
        let dom = smoothie.dom();
        dom.insert(String::from("Hello"), String::from("World!"));
    }

    thread::sleep(Duration::new(2, 0));

    smoothie.commit();

    thread::sleep(Duration::new(5, 0));

    let dom = smoothie.dom();
    println!("{:?}", dom);
}
