#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;

    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        _ => compare_list(_first_list, _second_list),
    }
}

fn includes<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _first_list.windows(_second_list.len()).any(|list_section| {
        let valid = list_section.eq(_second_list);
        return valid;
    })
}

fn same_size<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _first_list.len() == _second_list.len()
}

fn compare_list<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if includes(_second_list, _first_list) {
        if same_size(_first_list, _second_list) {
            return Comparison::Equal;
        }
        return Comparison::Sublist;
    }

    if includes(_first_list, _second_list) {
        if same_size(_first_list, _second_list) {
            return Comparison::Equal;
        }
        return Comparison::Superlist;
    }

    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }

    return Comparison::Unequal;
}
