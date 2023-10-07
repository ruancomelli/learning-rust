pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        [first, rest @ ..] => list
            .iter()
            .zip(rest.iter())
            .map(|(wanted_object, lost_object)| middle_sentence(wanted_object, lost_object))
            .chain(std::iter::once(final_sentence(first)))
            .collect(),
    }
}

fn middle_sentence(wanted_object: &str, lost_object: &str) -> String {
    format!(
        "For want of a {} the {} was lost.\n",
        wanted_object, lost_object
    )
}

fn final_sentence(wanted_object: &str) -> String {
    format!("And all for the want of a {}.", wanted_object)
}
