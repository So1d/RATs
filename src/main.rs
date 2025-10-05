use i3ipc::{
    I3Connection, I3EventListener, Subscription,
    event::{Event, inner::WindowChange},
};
use rats::{self, FocusedWindow};

fn main() {
    let mut listener = I3EventListener::connect().expect("Failed to connect event listener on wm");
    let mut connected = I3Connection::connect().expect("Failed to set connection with wm");

    listener.subscribe(&[Subscription::Window]).unwrap();
    println!("Hearing events...");

    for event in listener.listen() {
        if let Ok(Event::WindowEvent(eve)) = event {
            if eve.change == WindowChange::Focus {
                println!("New window has detected");
                let tree = connected.get_tree().expect("Failed do get tree");

                match rats::find_focused(&tree) {
                    FocusedWindow::Here(width, height) => {
                        println!("width: {} height: {} ", width, height);
                        if width < height {
                            connected
                                .run_command("split v")
                                .expect("Falha ao splitar v");
                            println!("splitamo v");
                        } else {
                            connected
                                .run_command("split h")
                                .expect("Falha ao splitar h");
                            println!("splitamo h");
                        }
                    }

                    FocusedWindow::NoOne => {
                        connected
                            .run_command("nop")
                            .expect("Nemhuma janela encontrada");
                    }
                }
            }
        }
    }
}
