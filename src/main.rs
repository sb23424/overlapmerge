use std::{ env, fs, process};

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

fn overlap_merge_two_string(
    shorter_string: &str,
    longer_string: &str,
    prefix: &str,
    suffix: &str,
) -> String {
    let shorter_string_max_idx = shorter_string.char_indices().count();
    let longer_string_max_idx = longer_string.char_indices().count();

    let prefix_max_idx = prefix.char_indices().count();
    let suffix_max_idx = suffix.char_indices().count();

    if shorter_string_max_idx > longer_string_max_idx {
        return overlap_merge_two_string(longer_string, shorter_string, prefix, suffix);
    }

    if shorter_string_max_idx == longer_string_max_idx && shorter_string.eq(longer_string) {
        return format!("{}", longer_string);
    } else if suffix_max_idx == 0 && longer_string.starts_with(shorter_string) {
        return format!("{}{}", prefix, longer_string);
    } else if prefix_max_idx == 0 && longer_string.ends_with(shorter_string) {
        return format!("{}{}", longer_string, suffix);
    } else if prefix_max_idx == 0 && suffix_max_idx == 0 && longer_string.contains(shorter_string) {
        return format!("{}", longer_string);
    }

    if shorter_string_max_idx > 1 {
        
        
        let mut prefix_recursion_result = "".to_string();
        let mut suffix_recursion_result = "".to_string();

        if prefix_max_idx == 0 {
            // there is not a prefix, so it's safe to try suffix recursion 
            let (short_string_suffix_slice_idx, _) = shorter_string
                .char_indices()
                .nth(shorter_string_max_idx - 1)
                .unwrap();
            let short_string_suffix_slice = &shorter_string[short_string_suffix_slice_idx..];
            let short_string_suffix_remainder_slice = &shorter_string[..short_string_suffix_slice_idx];

            suffix_recursion_result = overlap_merge_two_string(
                short_string_suffix_remainder_slice,
                longer_string,
                "",
                short_string_suffix_slice,
            );
        }

        if suffix_max_idx == 0 {
            // there is not a suffix, so it's safe to try prefix recursion 
            let (short_string_prefix_slice_idx, _) = shorter_string.char_indices().nth(1).unwrap();
        
            let short_string_prefix_slice = &shorter_string[..short_string_prefix_slice_idx];
            let short_string_prefix_remainder_slice = &shorter_string[short_string_prefix_slice_idx..];    

            prefix_recursion_result = overlap_merge_two_string(
                short_string_prefix_remainder_slice,
                longer_string,
                short_string_prefix_slice,
                "",
            );
        }

        let prefix_recursion_result_str = prefix_recursion_result.as_str();
        let suffix_recursion_result_str = suffix_recursion_result.as_str();

        let prefix_recursion_result_char_count = prefix_recursion_result_str.char_indices().count();
        let suffix_recursion_result_char_count = suffix_recursion_result_str.char_indices().count();

        if prefix_recursion_result_char_count == 0 {
            return suffix_recursion_result;
        }
        else if suffix_recursion_result_char_count == 0 {
            return prefix_recursion_result;
        }
        else if prefix_recursion_result_char_count < suffix_recursion_result_char_count {
            return prefix_recursion_result;
        }
        else{
            return suffix_recursion_result;
        }
    }

    return "".to_string();
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

    //println!("{}{}",file_1_data_str,file_2_data_str);
}

#[test]
fn test_merge_a() {
    let str_a = r#"abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_a, &str_b, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str,str_b);
}

#[test]
fn test_merge_b() {
    let str_a = r#"abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str,str_b);
}

#[test]
fn test_merge_c() {
    let str_a = r#"abcx"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str,"");
}

#[test]
fn test_merge_d() {
    let str_a = r#"xabc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str,"xabcde");
}

#[test]
fn test_merge_e() {
    let str_a = r#"🙂abc"#;
    let str_b = r#"abcde"#;

    let om_res = overlap_merge_two_string(&str_b, &str_a, "", "");
    let om_res_str = om_res.as_str();
    assert_eq!(om_res_str,"🙂abcde");
}


