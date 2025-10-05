use i3ipc::{
    event::{inner::WindowChange, Event},
    I3Connection, I3EventListener, Subscription,
};
use rats::{self, FocusedWindow};

fn main() {
    let mut listener = I3EventListener::connect().unwrap();
    let mut connected = I3Connection::connect().unwrap();

    listener.subscribe(&[Subscription::Window]).unwrap();
    println!("Hearing events...");

    for event in listener.listen() {
        match event.unwrap() {
            Event::WindowEvent(eve) => match eve.change {
                WindowChange::New => {
                    println!("Sei la porra, mas acho que tem janela nova ae");
                    let tree = connected.get_tree().expect("Failed do get tree");

                    match rats::find_focused(&tree) {
                        FocusedWindow::Here(width, height) => {
                            println!("Passamo do find focused");
                            println!("width: {} height: {}", width, height);
                            if width == 0 {
                                println!("Problemas mesmo no width")
                            };
                            if height == 0 {
                                println!("Problemas mesmo no height")
                            };
                            if width < height {
                                connected
                                    .run_command("split v")
                                    .expect("Falha ao splitar v");
                                println!("splitamo v");
                            } else if height < width {
                                connected
                                    .run_command("split h")
                                    .expect("Falha ao splitar h");
                                println!("splitamo h");
                            } else if height == width {
                                connected
                                    .run_command("split h")
                                    .expect("Falha ao splitar h");
                                println!("splitamo h");
                            };
                        }

                        FocusedWindow::NoOne => {
                            connected
                                .run_command("nop")
                                .expect("Nemhuma janela encontrada");
                        }
                    };
                }
                _ => {}
            },
            _ => {}
        }
    }
}
