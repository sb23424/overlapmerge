use std::{env, fs, process};

fn show_usage() {
    println!(
        r#"
    
    usage: overlapmerge <file1> <file2>
    
    outputs merged result with largest contiguous prefix/suffix style overlap.

    example: "hello world" "world peace" > "hello world peace"

    if no overlap is found, then nothing is output

    "#
    );
}

fn check_for_immediate_overlap(
    shorter_string: &str,
    longer_string: &str,
    prefix: &str,
    suffix: &str,
) -> Option<String> {
    let shorter_string_max_idx = shorter_string.char_indices().count();
    let longer_string_max_idx = longer_string.char_indices().count();

    let prefix_max_idx = prefix.char_indices().count();
    let suffix_max_idx = suffix.char_indices().count();

    if shorter_string_max_idx > longer_string_max_idx {
        return check_for_immediate_overlap(longer_string, shorter_string, prefix, suffix);
    }

    let mut res_string_option: Option<String> = None;

    if shorter_string_max_idx == longer_string_max_idx && shorter_string.eq(longer_string) {
        res_string_option = Some(format!("{}", longer_string));
    } else if suffix_max_idx == 0 && longer_string.starts_with(shorter_string) {
        res_string_option = Some(format!("{}{}", prefix, longer_string));
    } else if prefix_max_idx == 0 && longer_string.ends_with(shorter_string) {
        res_string_option = Some(format!("{}{}", longer_string, suffix));
    } else if prefix_max_idx == 0 && suffix_max_idx == 0 && longer_string.contains(shorter_string) {
        res_string_option = Some(format!("{}", longer_string));
    }

    return res_string_option;
}

fn overlap_merge_two_string(
    shorter_string: &str,
    longer_string: &str,
    prefix: &str,
    suffix: &str,
) -> String {
    let shorter_string_max_idx = shorter_string.char_indices().count();
    let longer_string_max_idx = longer_string.char_indices().count();

    if shorter_string_max_idx > longer_string_max_idx {
        return overlap_merge_two_string(longer_string, shorter_string, prefix, suffix);
    }

    let mut best_match = "".to_string();

    let max_iter_idx = shorter_string_max_idx - 1;

    for short_idx in 0..max_iter_idx {
        //prefix splitter
        let (short_string_prefix_slice_idx, _) =
            shorter_string.char_indices().nth(short_idx).unwrap();
        let short_string_prefix_slice = &shorter_string[..short_string_prefix_slice_idx];
        let short_string_prefix_remainder_slice = &shorter_string[short_string_prefix_slice_idx..];

        //suffix splitter
        let (short_string_suffix_slice_idx, _) = shorter_string
            .char_indices()
            .nth(max_iter_idx - short_idx)
            .unwrap();
        let short_string_suffix_slice = &shorter_string[short_string_suffix_slice_idx..];
        let short_string_suffix_remainder_slice = &shorter_string[..short_string_suffix_slice_idx];

        let prefix_immediate_overlap = check_for_immediate_overlap(
            short_string_prefix_remainder_slice,
            longer_string,
            short_string_prefix_slice,
            "",
        );

        let suffix_immediate_overlap = check_for_immediate_overlap(
            short_string_suffix_remainder_slice,
            longer_string,
            "",
            short_string_suffix_slice,
        );

        if prefix_immediate_overlap.is_some() && suffix_immediate_overlap.is_some() {
            let prefix_immediate_overlap_string = prefix_immediate_overlap.unwrap();
            let suffix_immediate_overlap_string = suffix_immediate_overlap.unwrap();

            let prefix_immediate_char_count =
                prefix_immediate_overlap_string.char_indices().count();
            let suffix_immediate_char_count =
                suffix_immediate_overlap_string.char_indices().count();

            if prefix_immediate_char_count == suffix_immediate_char_count {
                best_match = prefix_immediate_overlap_string;
                break;
            } else if prefix_immediate_char_count < suffix_immediate_char_count {
                best_match = prefix_immediate_overlap_string;
                break;
            } else {
                best_match = suffix_immediate_overlap_string;
                break;
            }
        } else if prefix_immediate_overlap.is_some() {
            let prefix_immediate_overlap_string = prefix_immediate_overlap.unwrap();
            best_match = prefix_immediate_overlap_string;
            break;
        } else if suffix_immediate_overlap.is_some() {
            let suffix_immediate_overlap_string = suffix_immediate_overlap.unwrap();
            best_match = suffix_immediate_overlap_string;
            break;
        }

        println!("{}", short_idx);
    }

    return best_match;
   
}

fn main() {
    if env::args().len() != 3 {
        show_usage();
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    let file_1_path = &args[1];
    let file_2_path = &args[2];

    let file_1_data =
        fs::read_to_string(file_1_path).expect(&format!("Unable to read file: {}", file_1_path));
    let file_2_data =
        fs::read_to_string(file_2_path).expect(&format!("Unable to read file: {}", file_2_path));

    let file_1_data_str = file_1_data.as_str();
    let file_2_data_str = file_2_data.as_str();

    let om_res = overlap_merge_two_string(file_1_data_str, file_2_data_str, "", "");

    println!("{}", om_res);
    process::exit(0);

}

#[test]
fn test_merge_a() {
    let str_a = r#"abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_a, &str_b, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, str_b);
}

#[test]
fn test_merge_b() {
    let str_a = r#"abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, str_b);
}

#[test]
fn test_merge_c() {
    let str_a = r#"abcx"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, "");
}

#[test]
fn test_merge_d() {
    let str_a = r#"xabc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, "xabcde");
}

#[test]
fn test_merge_e() {
    let str_a = r#"🙂abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, "🙂abcde");
}

#[test]
fn test_merge_f() {
    let str_a = r#"cdef"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str, "abcdef");
}
