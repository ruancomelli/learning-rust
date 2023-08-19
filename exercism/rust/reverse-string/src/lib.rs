use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let graphemes = input.graphemes(true);
    let mut output = String::new();

    for c in graphemes.rev() {
        output.push_str(c);
    }

    return output;
}
