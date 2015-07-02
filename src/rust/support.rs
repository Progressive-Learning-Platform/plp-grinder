pub fn slice_of(string: &str, range: (usize, usize)) -> String
{
    let native_slice = &string[range.0..range.1];
    let mut slice = String::new();
    slice.push_str(native_slice);
    slice
}
