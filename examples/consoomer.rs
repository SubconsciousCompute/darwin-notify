fn main() {
    darwin_notify::notify_register("tech.dafunk.net", |token| {
        println!("Got a notification. The token is {token}")
    })
    .unwrap();

    unsafe { darwin_notify::CFRunLoopRun() }
}
