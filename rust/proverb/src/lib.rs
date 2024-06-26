use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|items| format!("For want of a {} the {} was lost.\n", items[0], items[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}
