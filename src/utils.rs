pub fn gen_copy_func(hex_data: &str) -> String {
    format!("navigator.clipboard.writeText('{}');", hex_data)
}
