use i3ipc::{
    I3Connection, I3EventListener, Subscription,
    event::{Event, inner::WindowChange},
    reply::{self, NodeLayout, NodeType},
};

// enum FocusedWindow {
//     Here(i32, i32),
//     NoOne,
// }

fn main() {
    let mut listener = I3EventListener::connect().expect("Failed to connect event listener on wm");
    let mut connected = I3Connection::connect().expect("Failed to set up connection with wm");

    listener
        .subscribe(&[Subscription::Window])
        .expect("Failed to subscribe event listener");
    println!("Hearing events...");

    for event in listener.listen() {
        if let Ok(Event::WindowEvent(eve)) = event {
            if eve.change == WindowChange::Focus {
                println!("New focus has detected");

                let tree = connected.get_tree().expect("Failed to get tree");

                match find_focused(&tree) {
                    Some((width, height)) => {
                        println!("width: {} height: {} ", width, height);

                        let mut cmd = String::new();

                        if width < height {
                            cmd.push_str("split v");
                            println!("Splited vertically");
                        } else {
                            cmd.push_str("split h");
                            println!("Splited horizontally");
                        }
                        connected.run_command(&cmd).expect("Failed to split");
                    }

                    None => {
                        println!("There is no focalized window");
                    }
                }
            }
        }
    }
}
fn find_focused(tree: &reply::Node) -> Option<(i32, i32)> {
    if tree.focused {
        println!("This window is focused");

        let window = tree.window_rect;

        return Some((window.2, window.3));
    } else {
        for node in &tree.nodes {
            println!("This window is {:#?}", node.nodetype);

            if matches!(node.nodetype, NodeType::FloatingCon | NodeType::DockArea) {
                println!("This window is {:#?}, not a con", node.nodetype);
                continue;
            }

            if matches!(
                node.layout,
                NodeLayout::Tabbed | NodeLayout::DockArea | NodeLayout::Stacked
            ) {
                println!("This window is {:#?}, not a splitable layout", node.layout);
                continue;
            }

            let result = find_focused(&node);

            if let Some((width, height)) = result {
                return Some((width, height));
            }
        }
    }
    None
}
