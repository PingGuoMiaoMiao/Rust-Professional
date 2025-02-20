pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // Split the input string into the number and its base
    let parts: Vec<&str> = num_str.split('(').collect();
    let num_part = parts[0];
    let base_part = parts[1].trim_end_matches(')');

    // Parse the base of the input number
    let from_base: u32 = base_part.parse().expect("Invalid base");

    // Convert the input number from its base to base-10
    let num_in_base_10 = u32::from_str_radix(num_part, from_base).expect("Invalid number");

    // Convert the base-10 number to the desired base
    let result = format!("{:X}", num_in_base_10)
        .to_lowercase()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    result
}
