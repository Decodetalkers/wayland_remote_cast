use ws::listen;
//use std::cell::RefCell;
//thread_local! {
//    static GLOBAL: RefCell<Option<Vec<u8>>> = RefCell::new(None);
//}
fn main() {
    listen("127.0.0.1:3012", |out| {
        move |msg| {
            //println!("ssss");
            println!("received msg: {}", msg);

            let out2 = out.clone();

            std::thread::spawn(move ||{
                if let ws::Message::Binary(brinary) = msg {
                    out2.broadcast(ws::Message::Binary(brinary)).unwrap();
                } else {
                    out2.broadcast("continue").unwrap();
                }
            });

           Ok(())
        }
    })
    .unwrap()
}
