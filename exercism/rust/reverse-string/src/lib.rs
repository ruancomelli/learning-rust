use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let graphemes = input.graphemes(true).collect::<Vec<&str>>();
    let mut output = String::new();

    for c in graphemes.iter().rev() {
        output.push_str(c);
    }

    return output;
}
