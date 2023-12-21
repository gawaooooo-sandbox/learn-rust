pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            // @see https://stackoverflow.com/a/63437864
            if let Err(e) = writeln!(writer, "{}", line) {
                eprintln!("Writing error: {}", e);
            }
        }
    }
}
