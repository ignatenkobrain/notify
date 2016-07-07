extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::path::Path;
use std::sync::mpsc::channel;

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(path, RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
          Ok(notify::Event{path:Some(path), op:Ok(op)}) => {
              println!("{:?} {:?}", op, path);
          },
          Err(e) => println!("watch error {}", e),
          _ => ()
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    println!("watching {}", path);
    if let Err(err) = watch(path) {
      println!("Error!: {:?}", err)
    }
}
