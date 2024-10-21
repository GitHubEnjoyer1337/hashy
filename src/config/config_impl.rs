

#[derive(Debug)]
pub struct Config<'a> {
    pub flag: Option<&'a str>,
    pub to_hash: &'a str,
    pub count: usize,
}





impl<'a> Config<'a> {
    pub fn build(
        flag: Option<&'a str>, 
        to_hash: &'a str, 
        count_str: &str
        ) -> Result<Config<'a>, String> {

        let flags: Vec<&str> = vec!["-a", "-b"];

        let flag = if let Some(f) = flag {
            if flags.contains(&f) {
                Some(f)
            } else {
                return Err("invalid flag".to_string());
            }
        } else {
            None
        };

        let count = match count_str.parse() {
            Ok(num) => num,
            Err(_) => return Err("second arg must be valid (usize)".to_string()),
        };
        Ok(Config {flag, to_hash, count})
    }
}
