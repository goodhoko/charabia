use finl_unicode::categories::CharacterCategories;
use slice_group_by::StrGroupBy;

pub(crate) fn split_camel_case_bounds(str: &str) -> impl Iterator<Item = &str> {
    let mut last_char_was_lowercase = false;

    str.linear_group_by(move |a, b| {
        if b.is_mark_nonspacing() {
            return true;
        }

        if (last_char_was_lowercase || a.is_letter_lowercase()) && b.is_letter_uppercase() {
            return false;
        }

        last_char_was_lowercase = b.is_letter_lowercase();
        true
    })
}

#[cfg(test)]
mod test {
    use super::split_camel_case_bounds;

    macro_rules! test_segmentation {
        ($text:expr, $segmented:expr, $name:ident) => {
            #[test]
            fn $name() {
                let segmented_text: Vec<_> = split_camel_case_bounds($text).collect();
                assert_eq!(segmented_text, $segmented);
            }
        };
    }

    test_segmentation!("", [""], empty_string_is_preserved);
    test_segmentation!("a", ["a"], one_letter_word);
    test_segmentation!("aB", ["a", "B"], two_letter_word);
    test_segmentation!("camelCase", ["camel", "Case"], camel_case_is_split);
    test_segmentation!("SCREAMING", ["SCREAMING"], all_caps_is_not_split);
    test_segmentation!("resuméWriter", ["resumé", "Writer"], non_ascii_boundary_on_left);
    test_segmentation!("KarelČapek", ["Karel", "Čapek"], non_ascii_boundary_on_right);
    test_segmentation!("resuméWriter", ["Karel", "Čapek"], capek_hard);
    test_segmentation!(
        "resume\u{0301}Writer",
        ["resume\u{0301}", "Writer"],
        non_spacing_marks_are_respected
    );
}
