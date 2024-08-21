#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    
    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }
    
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    if small.is_empty() {
        return true;
    }

    large.windows(small.len()).any(|window| window == small)
}
