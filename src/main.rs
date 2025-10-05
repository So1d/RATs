use i3ipc::I3Connection;
use i3ipc::event::inner::WindowChange;
use i3ipc::reply;
use i3ipc::{I3EventListener, Subscription, event::Event};

enum FocusedWindow {
    Here(i32, i32),
    NoOne,
}

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

                    match find_focused(&tree) {
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
                            } else {
                                println!("Algo deu errado");
                            }
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
fn find_focused(tree: &reply::Node) -> FocusedWindow {
    if tree.focused {
        println!("Certamente encontramos a janela focada");
        let window = tree.window_rect;
        return FocusedWindow::Here(window.2, window.3);
    } else {
        for node in &tree.nodes {
            if let FocusedWindow::Here(width, height) = find_focused(&node) {
                return FocusedWindow::Here(width, height);
            }
        }
    }
    println!("Ou deu merda ou nao achamo nemhuma focada");
    FocusedWindow::NoOne
}
