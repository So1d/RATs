use i3ipc::{
    I3Connection, I3EventListener, Subscription,
    event::{Event, inner::WindowChange},
    reply::{self, NodeLayout, NodeType},
};

pub enum FocusedWindow {
    Here(i32, i32),
    NoOne,
}

fn main() {
    let mut listener = I3EventListener::connect().expect("Failed to connect event listener on wm");
    let mut connected = I3Connection::connect().expect("Failed to set connection with wm");

    listener.subscribe(&[Subscription::Window]).unwrap();
    println!("Hearing events...");

    for event in listener.listen() {
        if let Ok(Event::WindowEvent(eve)) = event {
            if eve.change == WindowChange::Focus {
                println!("New focus has detected");
                let tree = connected.get_tree().expect("Failed to get tree");

                match find_focused(&tree) {
                    FocusedWindow::Here(width, height) => {
                        println!("width: {} height: {} ", width, height);
                        if width < height {
                            connected
                                .run_command("split v")
                                .expect("Failed to split vertically");
                            println!("Splited vertically");
                        } else {
                            connected
                                .run_command("split h")
                                .expect("Failed to split horizontally");
                            println!("Splited horizontally");
                        }
                    }

                    FocusedWindow::NoOne => {
                        println!("There is no focalized window");
                    }
                }
            }
        }
    }
}
fn find_focused(tree: &reply::Node) -> FocusedWindow {
    if tree.focused {
        println!("This window is focused");
        let window = tree.window_rect;
        return FocusedWindow::Here(window.2, window.3);
    } else {
        for node in &tree.nodes {
            println!("This window is {:#?}", node.nodetype);

            if matches!(node.nodetype, NodeType::FloatingCon | NodeType::DockArea) {
                println!("This window is {:#?}, not a con", node.nodetype);
                break;
            }

            if matches!(
                node.layout,
                NodeLayout::Tabbed | NodeLayout::DockArea | NodeLayout::Stacked
            ) {
                println!("This window is {:#?}, not a splitable layout", node.layout);
                break;
            }

            let result = find_focused(&node);
            if let FocusedWindow::Here(width, height) = result {
                return FocusedWindow::Here(width, height);
            }
        }
    }
    FocusedWindow::NoOne
}
