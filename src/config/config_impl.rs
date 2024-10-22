

#[derive(Debug)]
pub struct Config<'a> {
    pub flag: Option<&'a str>,
    pub to_hash: &'a str,
    pub count: usize,
}





impl<'a> Config<'a> {
    pub fn build(stringvec: &'a [String]
        ) -> Result<Config, Box<dyn std::error::Error>> {
        
        if stringvec.len() < 3 {
            return Err("Usage: <program> [flag] <to_hash> <count>".into());
        }
        let flags: Vec<&str> = vec!["-a", "-b"];

        let flag: Option<&str>;
        let to_hash: &str;
        let count: usize;

        if stringvec.len() >= 4 && flags.contains(&stringvec[1].as_str()) {
            flag = Some(&stringvec[1]);
            to_hash = &stringvec[2];
            count = match stringvec[3].parse() {
                Ok(num) => num,
                Err(_) => return Err("second arg must be valid (usize)".into()),
            };
        } else {
            flag = None;
            to_hash = &stringvec[1];
            count = match stringvec[2].parse() {
                Ok(num) => num,
                Err(_) => return Err("second arg must be valid (usize)".into()),
            };
        }
        Ok(Config {flag, to_hash, count})
    }
}

































