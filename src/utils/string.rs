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
