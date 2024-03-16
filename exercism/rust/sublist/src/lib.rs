#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() > second_list.len() {
        match is_contiguous_subsequence(second_list, first_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        }
    } else if first_list.len() < second_list.len() {
        match is_contiguous_subsequence(first_list, second_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        }
    } else {
        match first_list == second_list {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        }
    }
}

fn is_contiguous_subsequence<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    match first_list.len() {
        0 => true,
        _ => second_list
            .windows(first_list.len())
            .any(|window| window == first_list),
    }
}
