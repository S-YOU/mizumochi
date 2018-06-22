use std::fmt;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub duration: Duration,
    pub frequency: Duration,
    pub operations: Vec<Operation>,
    pub speed: Speed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Speed {
    Bps(usize),
    PassThrough,
}

impl FromStr for Speed {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::error::Error;

        if s == "pass_through" {
            Ok(Speed::PassThrough)
        } else if s.ends_with("Bps") {
            let (n, _) = s.split_at(s.len() - 3);
            let mut s = n.to_string();

            let scale: usize = match s.pop().ok_or("Invalid speed")? {
                'K' => 1 << 10,
                'M' => 1 << 20,
                'G' => 1 << 30,
                r => {
                    s.push(r);
                    1
                }
            };

            let speed = s.parse::<usize>().map_err(|e| e.description().to_string())?;
            let speed = speed.checked_mul(scale).ok_or("overflow")?;

            Ok(Speed::Bps(speed))
        } else {
            let speed = s.parse::<usize>().map_err(|e| e.description().to_string())?;

            Ok(Speed::Bps(speed))
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Speed::Bps(bps) if bps < 1 << 10 => write!(f, "{}Bps", bps),
            Speed::Bps(bps) if bps < 1 << 20 => write!(f, "{}KBps", bps as f64 / (1 << 10) as f64),
            Speed::Bps(bps) if bps < 1 << 30 => write!(f, "{}MBps", bps as f64 / (1 << 20) as f64),
            Speed::Bps(bps) => write!(f, "{}GBps", bps as f64 / (1 << 30) as f64),
            Speed::PassThrough => write!(f, "PassThrough"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Read,
    Write,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Read => write!(f, "Read"),
            Operation::Write => write!(f, "Write"),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            duration: Duration::from_secs(10 * 60),
            frequency: Duration::from_secs(30 * 60),
            operations: vec![Operation::Read, Operation::Write],
            speed: Speed::PassThrough,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let d = self.duration.as_secs();
        let f = self.frequency.as_secs();
        let ops = self
            .operations
            .iter()
            .fold(Vec::new(), |mut acc, x| {
                acc.push(format!("{}", x).to_string());
                acc
            })
            .join(":");
        write!(
            fmt,
            "Config {{Duration: {}sec, Frequency: {}sec, Operations: {}, Speed: {}}}",
            d, f, ops, self.speed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_from_str() {
        assert!(Speed::from_str("").is_err());
        assert!(Speed::from_str("alskjaslkdfjhasjdhfb").is_err());
        assert!(Speed::from_str("Bps").is_err());
        assert_eq!(Ok(Speed::Bps(1 << 10)), Speed::from_str("1024"));
        assert_eq!(Ok(Speed::Bps(1 << 10)), Speed::from_str("1024Bps"));
        assert_eq!(Ok(Speed::Bps(1 << 20)), Speed::from_str("1024KBps"));
        assert_eq!(Ok(Speed::Bps(1 << 30)), Speed::from_str("1024MBps"));
        assert_eq!(Ok(Speed::Bps(1 << 40)), Speed::from_str("1024GBps"));
    }
}
