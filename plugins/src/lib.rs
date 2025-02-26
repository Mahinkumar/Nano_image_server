

pub fn log() {
    println!("Plugins are enabled!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        log();
    }
}