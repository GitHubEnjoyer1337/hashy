

#[derive(Debug)]
pub struct Config<'a> {
    pub flag: Option<Flag>,
    pub to_hash: &'a str,
    pub count: usize,
    pub query: Option<&'a str>,
    pub hash_start: Option<&'a str>,
    pub hash_end: Option<&'a str>,
}


#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flag {
    A,
    B,
    C,
    S,
}


impl Flag {
    fn from_str(s: &str) -> Option<Flag> {
        match s {
            "-a" => Some(Flag::A),
            "-b" => Some(Flag::B),
            "-c" => Some(Flag::C),
            "-s" => Some(Flag::S),
            _ => None,
        }
    }
}

impl<'a> Config<'a> {
    pub fn build(stringvec: &'a [String]) -> Result<Config<'a>, Box<dyn std::error::Error>> {
        if stringvec.len() < 3 {
            return Err("Usage: <program> [flag] <to_hash> <count>".into());
        }

        let (flag, to_hash, count, query, hash_start, hash_end) = if stringvec.len() >= 4 {
            if let Some(flag) = Flag::from_str(&stringvec[1]) {
                match flag {
                    Flag::C => {
                        if stringvec.len() < 6 {
                            return Err("Not enough args for -c flag".into());
                        }
                        (
                            Some(flag),
                            stringvec[2].as_str(),
                            Self::parse_count(&stringvec[3])?,
                            None,
                            Some(stringvec[4].as_str()),
                            Some(stringvec[5].as_str())
                        )
                    },
                    Flag::S => {
                        if stringvec.len() < 5 {
                            return Err("Not enough args for -s flag".into());
                        }
                        (
                            Some(flag),
                            stringvec[3].as_str(),
                            Self::parse_count(&stringvec[4])?,
                            Some(stringvec[2].as_str()),
                            None,
                            None
                        )
                    },
                    _ => (
                            Some(flag),
                            stringvec[2].as_str(),
                            Self::parse_count(&stringvec[3])?,
                            None,
                            None,
                            None
                         )
                    }
                } else {
                    (
                            None,
                            stringvec[1].as_str(),
                            Self::parse_count(&stringvec[2])?,
                            None,
                            None,
                            None
                    )
                }
        } else {
            (
                            None,
                            stringvec[1].as_str(),
                            Self::parse_count(&stringvec[2])?,
                            None,
                            None,
                            None
            )
        };
        Ok(Config { flag, to_hash, count, query, hash_start, hash_end})
    }
    pub fn parse_count(s: &str) -> Result<usize, Box<dyn std::error::Error>> {
        s.parse().map_err(|_| "count must be valid (usize)".into())
    } 
}

