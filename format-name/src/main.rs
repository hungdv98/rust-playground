fn format_name_original(name: &str) -> String {
    name.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let head = first.to_uppercase().collect::<String>();
                    let tail = chars.as_str().to_lowercase();
                    head + &tail
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_name_optimized(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut words = name.split_whitespace().peekable();

    while let Some(word) = words.next() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase());
            for c in chars {
                result.push(c.to_lowercase().next().unwrap());
            }
        }
        if words.peek().is_some() {
            result.push(' ');
        }
    }
    result
}
fn main() {
    let name: &str = " dAnG  ViET    hUNG   ";
    println!("Original: '{}'", format_name_original(name));
    println!("Optimized: '{}'", format_name_optimized(name));
}
