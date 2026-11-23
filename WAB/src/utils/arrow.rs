pub fn next(menu: Vec<String>, mut index: usize) -> usize {
    if index + 1 > menu.len() - 1 {
        index = 0;
    } else {
        index += 1;
    }

    index
}

pub fn prev(menu: Vec<String>, mut index: usize) -> usize {
    if index == 0 {
        index = menu.len() - 1;
    } else {
        index -= 1;
    }

    index
}