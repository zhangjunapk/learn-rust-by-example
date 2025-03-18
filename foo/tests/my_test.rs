//cargo test test_file
//cargo test [filter]
//filter 用于过滤出满足条件的方法并执行

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;

    #[test]
    fn test_file() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("foo.txt")
            .expect("Could not open foo.txt");
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to file");
        }
    }

    #[test]
    fn test_file_also() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("foo.txt")
            .expect("Could not open foo.txt");
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to file");
        }
    }
}
