use std::{cmp, env, fs, process};


fn show_usage(){

    println!(r#"
    
    usage: overlapmerge <file1> <file2>
    
    outputs merged result with largest contiguous prefix/suffix style overlap.

    example: "hello world" "world peace" > "hello world peace"

    if no overlap is found, then nothing is output

    "#);

}


fn overlap_merge_two_string(file_1_data_str: &str, file_2_data_str: &str) -> String {

    //dbg!(file_1_data_str);
    //dbg!(file_2_data_str);

    if file_1_data_str.eq(file_2_data_str) {
        return format!("{}",file_1_data_str);
        //return Ok(None);
        //process::exit(0);
    }
    

    let file_1_data_str_max_idx = file_1_data_str.char_indices().count();
    let file_2_data_str_max_idx = file_2_data_str.char_indices().count();

    let min_str_len = cmp::min(file_1_data_str_max_idx, file_2_data_str_max_idx);

    //dbg!(min_str_len);

    for cur_len in (1..min_str_len).rev(){
        //dbg!(cur_len);
        
        let (f1_sl_idx, _) = file_1_data_str.char_indices().nth(cur_len).unwrap();
        let (f2_sl_idx, _) = file_2_data_str.char_indices().nth(cur_len).unwrap();

        let f1_slice = &file_1_data_str[..f1_sl_idx];
        let f2_slice = &file_2_data_str[..f2_sl_idx];

        if file_2_data_str.ends_with(f1_slice){
            //dbg!(f1_slice);
            return format!("{}{}",file_2_data_str,&file_1_data_str[f1_sl_idx..]);
            //process::exit(0);
        }
        else if file_1_data_str.ends_with(f2_slice) {
            //dbg!(f2_slice);
            return format!("{}{}",file_1_data_str,&file_2_data_str[f2_sl_idx..]);
            //process::exit(0);
        }
        else if file_2_data_str.starts_with(f1_slice){
            //dbg!(f1_slice);
            return format!("{}",file_2_data_str);
            //process::exit(0);
        }
        else if file_1_data_str.starts_with(f2_slice) {
            //dbg!(f2_slice);
            return format!("{}",file_1_data_str);
            //process::exit(0);
        }

    } 

    return "".to_string();

}


fn main() {

    if env::args().len() != 3 
    {
        show_usage();
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    
    let file_1_path = &args[1];
    let file_2_path = &args[2];
    
    //dbg!(file_1_path);
    //dbg!(file_2_path);

    let file_1_data = fs::read_to_string(file_1_path).expect(&format!("Unable to read file: {}", file_1_path));
    let file_2_data = fs::read_to_string(file_2_path).expect(&format!("Unable to read file: {}", file_2_path));

    let file_1_data_str =  file_1_data.as_str();
    let file_2_data_str =  file_2_data.as_str();

    let om_res = overlap_merge_two_string(file_1_data_str,file_2_data_str);

    println!("{}",om_res);
    process::exit(0);


    //println!("{}{}",file_1_data_str,file_2_data_str);

}

#[test]
fn test_merge(){

    let str_a = r#"a
b
c"#;

let str_b = r#"a
b
c
d
e"#;

let om_res = overlap_merge_two_string(&str_a, &str_b);


assert_eq!(str_b,om_res);


}