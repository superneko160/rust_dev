use super::*;

#[test]
fn test_get_text() {
    let text_path = Path::new(&"/workspaces/rust_dev/freqency-analysis/src/sample.txt");
    let result = String::from("ABa　あCDF/$XYZ XAXXX");
    assert_eq!(result, get_text(text_path));
}

#[test]
fn test_freqency_analysis() {
    let file_contents = String::from("aBBc");
    let result = HashMap::from([('a',1),('B',2),('c',1)]);
    assert_eq!(result, freqency_analysis(file_contents));
}

#[test]
fn test_sort_dict() {
    let dict = HashMap::from([('a',2),('b',1),('c',3)]);
    let result: Vec<(char, u32)> = vec![('c',3),('a',2),('b',1)];
    assert_eq!(result, sort_dict(dict));
}

#[test]
fn test_count_all_char() {
    let vec: Vec<(char, u32)> = vec![('c',3),('a',2),('b',1)];
    let result = 6;
    assert_eq!(result, count_all_char(&vec));
}

#[test]
fn test_calc_rate() {
    let result: f32 = 24.0;
    assert_eq!(result, calc_rate(12 as f32, 50 as f32));
}