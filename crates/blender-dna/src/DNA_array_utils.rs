//! Safe Rust implementation of blender::dna::array utility functions

/// Removes an element from a vector and shifts elements after it towards the front.
pub fn remove_index<T>(
    items: &mut Vec<T>,
    active_index: Option<&mut i32>,
    index: usize,
    mut destruct_item: impl FnMut(&mut T),
) {
    if index >= items.len() {
        return;
    }
    let mut item = items.remove(index);
    destruct_item(&mut item);

    if let Some(active) = active_index {
        let old_active = *active;
        let new_items_num = items.len() as i32;
        let new_active = if new_items_num == 0 {
            0
        } else if old_active >= new_items_num {
            new_items_num - 1
        } else {
            old_active
        };
        *active = new_active.max(0);
    }
}

/// Removes all elements for which the predicate is true.
pub fn remove_if<T>(
    items: &mut Vec<T>,
    mut predicate: impl FnMut(&T) -> bool,
    mut destruct_item: impl FnMut(&mut T),
) {
    let mut i = 0;
    while i < items.len() {
        if predicate(&items[i]) {
            let mut item = items.remove(i);
            destruct_item(&mut item);
        } else {
            i += 1;
        }
    }
}

/// Removes all elements from an array and frees it.
pub fn clear<T>(
    items: &mut Vec<T>,
    active_index: Option<&mut i32>,
    mut destruct_item: impl FnMut(&mut T),
) {
    for mut item in items.drain(..) {
        destruct_item(&mut item);
    }
    if let Some(active) = active_index {
        *active = 0;
    }
}

/// Moves one element from one index to another, shifting intermediate elements.
pub fn move_index<T>(items: &mut [T], from_index: usize, to_index: usize) {
    if from_index == to_index || from_index >= items.len() || to_index >= items.len() {
        return;
    }
    if from_index < to_index {
        items[from_index..=to_index].rotate_left(1);
    } else {
        items[to_index..=from_index].rotate_right(1);
    }
}
