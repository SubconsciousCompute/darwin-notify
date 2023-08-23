fn main() {
    darwin_notify::notify_register("tech.subcom.darwin-notify", |token| {
        println!("Got a notification. The token is {token}")
    })
    .unwrap();

    unsafe { darwin_notify::CFRunLoopRun() }
}
