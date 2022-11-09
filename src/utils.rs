

pub fn get_lines(string: &str, line_number: usize, radius: usize) -> String {
    let mut lines_to_print: Vec<String> = Vec::new();

    for (index, line) in string.lines().enumerate() {

        // Skip the lines before
        if index < line_number - radius {
            continue;
        }

        // Stop after the lines after
        if index > line_number + radius {
            break;
        }

        if index == line_number {
            lines_to_print.push(format!("> {}", line));
        } else {
            lines_to_print.push(format!("  {}", line));
        }

    }
        
    lines_to_print.join("\n")
}

