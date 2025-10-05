use i3ipc::reply;

pub enum FocusedWindow {
    Here(i32, i32),
    NoOne,
}

pub fn find_focused(tree: &reply::Node) -> FocusedWindow {
    if tree.focused {
        let window = tree.window_rect;
        return FocusedWindow::Here(window.2, window.3);
    } else {
        for node in &tree.nodes {
            let result = find_focused(&node);
            if let FocusedWindow::Here(width, height) = result {
                return FocusedWindow::Here(width, height);
            }
        }
    }
    FocusedWindow::NoOne
}
