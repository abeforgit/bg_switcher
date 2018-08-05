extern crate daemonize;
extern crate i3ipc;

use i3ipc::event::inner::WorkspaceChange;
use i3ipc::event::Event;
use i3ipc::reply::Node;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use std::fs::File;
use std::process::Command;

use daemonize::Daemonize;

fn set_bg(workspace: Node) {
    let name: &str = &workspace.name.unwrap();
    let dir = "/home/arne/.background";
    Command::new("feh")
        .arg("--no-fehbg")
        .arg("--bg-fill")
        .arg(format!("{}/{}", dir, name))
        .spawn()
        .expect("Failed to start");
}

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
    .stdout(stdout)
    .stderr(stderr)
    .user("arne")
    .working_directory("/home/arne");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => println!("Error, {}", e),
    }
    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Workspace];
    listener.subscribe(&subs).unwrap();
    loop {
        for event in listener.listen() {
            match event.unwrap() {
                Event::WorkspaceEvent(e) => match e.change {
                    WorkspaceChange::Focus => set_bg(e.current.unwrap()),
                    _ => (),
                },
                _ => unreachable!(),
            }
        }
    }
}
