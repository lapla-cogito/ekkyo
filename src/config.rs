use anyhow::Context as _;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Mode {
    Active,
    Passive,
}

impl std::str::FromStr for Mode {
    type Err = crate::error::ConfigParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "passive" | "Passive" => Ok(Mode::Passive),
            "active" | "Active" => Ok(Mode::Active),
            _ => Err(crate::error::ConfigParseErr::from(anyhow::anyhow!(
                "cannot parse {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Config {
    pub local_as: crate::types::ASNum,
    pub local_ip: std::net::Ipv4Addr,
    pub remote_as: crate::types::ASNum,
    pub remote_ip: std::net::Ipv4Addr,
    pub mode: Mode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            local_as: crate::types::ASNum::from(64512),
            local_ip: std::net::Ipv4Addr::new(127, 0, 0, 1),
            remote_as: crate::types::ASNum::from(64513),
            remote_ip: std::net::Ipv4Addr::new(127, 0, 0, 2),
            mode: Mode::Active,
        }
    }
}

impl std::str::FromStr for Config {
    type Err = crate::error::ConfigParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: Vec<&str> = s.split(' ').collect();
        let local_as = crate::types::ASNum::from(config[0].parse::<u16>().context(format!(
            "cannot parse 1st part of config, `{0}`, \
                 as as-number and config is {1}",
            config[0], s
        ))?);
        let local_ip: std::net::Ipv4Addr = config[1].parse().context(format!(
            "cannot parse 2nd part of config, `{0}`, \
            as as-number and config is {1}",
            config[1], s
        ))?;
        let remote_as = crate::types::ASNum::from(config[2].parse::<u16>().context(format!(
            "cannot parse 3rd part of config, `{0}`, \
                 as as-number and config is {1}",
            config[2], s
        ))?);
        let remote_ip: std::net::Ipv4Addr = config[3].parse().context(format!(
            "cannot parse 4th part of config, `{0}`, \
             as as-number and config is {1}",
            config[3], s
        ))?;
        let mode: Mode = config[4].parse().context(format!(
            "cannot parse 5th part of config, `{0}`, \
             as as-number and config is {1}",
            config[4], s
        ))?;

        Ok(Self {
            local_as,
            local_ip,
            remote_as,
            remote_ip,
            mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_can_parse() {
        let config: Config = "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        assert_eq!(config.local_as, crate::types::ASNum::from(64512));
        assert_eq!(config.local_ip, std::net::Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(config.remote_as, crate::types::ASNum::from(65413));
        assert_eq!(config.remote_ip, std::net::Ipv4Addr::new(127, 0, 0, 2));
        assert_eq!(config.mode, Mode::Active);
    }

    #[test]
    fn invalid_config() {
        let config: Result<Config, crate::error::ConfigParseErr> = "foo bar baz qux quux".parse();
        assert!(config.is_err());
    }
}
