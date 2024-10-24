

#[derive(Debug)]
pub struct Config<'a> {
    pub flag: Option<&'a str>,
    pub to_hash: &'a str,
    pub count: usize,
    pub query: Option<&'a str>,
}



impl<'a> Config<'a> {
    pub fn build(stringvec: &'a [String]) -> Result<Config<'a>, Box<dyn std::error::Error>> {
        if stringvec.len() < 3 {
            return Err("Usage: <program> [flag] <to_hash> <count>".into());
        }

        let flags: Vec<&str> = vec!["-a", "-b", "-s"];

        let (flag, to_hash, count, query) = if stringvec.len() >= 4 && flags.contains(&stringvec[1].as_str()) {
            let flag = Some(stringvec[1].as_str());
            if flag == Some("-s") {
                if stringvec.len() < 5 {
                    return Err("Not enough arguments for -s flag".into());
                }
                (flag, stringvec[3].as_str(), Self::parse_count(&stringvec[4])?, Some(stringvec[2].as_str()))
            } else {
                (flag, stringvec[2].as_str(), Self::parse_count(&stringvec[3])?, None)
            }
        } else {
            (None, stringvec[1].as_str(), Self::parse_count(&stringvec[2])?, None)
        };

        Ok(Config { flag, to_hash, count, query })
    }

    fn parse_count(s: &str) -> Result<usize, Box<dyn std::error::Error>> {
        s.parse().map_err(|_| "count must be valid (usize)".into())
    }
}


/*impl<'a> Config<'a> {
    pub fn build(stringvec: &'a [String]
        ) -> Result<Config, Box<dyn std::error::Error>> {
        
        if stringvec.len() < 3 {
            return Err("Usage: <program> [flag] <to_hash> <count>".into());
        }
        let flags: Vec<&str> = vec!["-a", "-b", "s"];

        let flag: Option<&str>;
        let to_hash: &str;
        let count: usize;
        let query: Option<&str>;

        if stringvec.len() >= 4 && flags.contains(&stringvec[1].as_str()) {
            flag = Some(&stringvec[1]);
            if flag == Some("-s") {
                to_hash = &stringvec[3];
                count = match stringvec[4].parse() {
                    Ok(num) => num,
                    Err(_) => return Err("count must be valid (usize)".into()),
                };
                query = Some(&stringvec[2]);
                return Ok(Config {flag, to_hash, count, query})
            }
            to_hash = &stringvec[2];
            count = match stringvec[3].parse() {
                Ok(num) => num,
                Err(_) => return Err("count must be valid (usize)".into()),
            };
            query = None;
        } else {
            flag = None;
            to_hash = &stringvec[1];
            count = match stringvec[2].parse() {
                Ok(num) => num,
                Err(_) => return Err("count must be valid (usize)".into()),
            };
            query = None;
        }
        Ok(Config {flag, to_hash, count, query})
    }
}

*/































