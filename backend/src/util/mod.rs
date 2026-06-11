pub fn sanitize_ean(ean: &str) -> String {
    ean.trim()
        .trim_matches('\'')
        .trim_matches('"')
        .to_string()
}

pub fn clean_ocr_name(raw_name: &str) -> String {
    let intermediate = raw_name
        .to_uppercase()
        .replace('Ä', "AE")
        .replace('Ö', "OE")
        .replace('Ü', "UE")
        .replace('ß', "SS")
        .chars()
        .filter(|c| c.is_alphanumeric() || [' ', '-', '&', '.', ',', '%'].contains(c))
        .collect::<String>();

    intermediate
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
