#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (len_1, len_2) = (_first_list.len(), _second_list.len());

    if len_1 == 0 && len_2 == 0 {
        return Comparison::Equal;
    }
    if len_1 == 0 && len_2 > 0 {
        return Comparison::Sublist;
    }
    if len_1 > 0 && len_2 == 0 {
        return Comparison::Superlist;
    }

    let (super_list, sub_list, pre_result) = if len_1 < len_2 {
        (_second_list, _first_list, Comparison::Sublist)
    } else if len_1 > len_2 {
        (_first_list, _second_list, Comparison::Superlist)
    } else {
        (_first_list, _second_list, Comparison::Equal)
    };

    let mut super_list_count = 0;
    let mut sub_list_count = 0;

    while super_list_count < super_list.len() {
        if super_list.len() - super_list_count < sub_list.len() - sub_list_count {
            break;
        }

        if super_list[super_list_count] == sub_list[sub_list_count] {
            if sub_list_count == sub_list.len() - 1 {
                return pre_result;
            }
            sub_list_count += 1;
        } else if sub_list_count > 0 {
            super_list_count -= sub_list_count; // Rewind in case of partial sublisting.
            sub_list_count = 0;
        }
        super_list_count += 1;
    }

    return Comparison::Unequal;
}
