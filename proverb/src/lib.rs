pub fn build_proverb(list: Vec<&str>) -> String {
    let mut result = String::new();
    for index in 1..list.len() {
        result += &format!("For want of a {} the {} was lost.\n", list[index-1], list[index]);
    }
    if list.len() > 0 {
        result += &format!("And all for the want of a {}.", list[0]);
    }
    result
}
