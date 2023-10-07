pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        [first, rest @ ..] => {
            let mut proverb_lines = Vec::new();

            let mut pairs = list.iter().zip(rest.iter());
            for (wanted_object, lost_object) in pairs {
                proverb_lines.push(middle_sentence(wanted_object, lost_object));
            }
            proverb_lines.push(final_sentence(first));
            proverb_lines.join("\n")
        }
    }
}

fn middle_sentence(wanted_object: &str, lost_object: &str) -> String {
    format!(
        "For want of a {} the {} was lost.",
        wanted_object, lost_object
    )
}

fn final_sentence(wanted_object: &str) -> String {
    format!("And all for the want of a {}.", wanted_object)
}
