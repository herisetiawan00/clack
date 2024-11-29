use std::cmp::min;

pub fn split_with_space(input: String, length: usize, skip: Option<usize>) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < input.len() {
        let mut end = usize::min(
            start
                + if start < 1 {
                    length - skip.unwrap_or(0)
                } else {
                    length
                },
            input.len(),
        );
        if end < input.len() && &input[end..end + 1] != " " {
            if let Some(space_pos) = input[start..end].rfind(' ') {
                end = start + space_pos;
            }
        }

        chunks.push(input[start..end].trim().to_string());
        start = end + 1
    }

    chunks
}

pub fn split_text_with_custom_first(
    input: &str,
    first_length: usize,
    chunk_size: usize,
) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();

    // Handle the first chunk
    if first_length > 0 && !chars.is_empty() {
        let first_chunk: String = chars.iter().take(first_length).collect();
        result.push(first_chunk);
    }

    // Handle the remaining chunks
    let remaining_chars = &chars[min(first_length, chars.len())..];
    result.extend(
        remaining_chars
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect()),
    );

    result
}
