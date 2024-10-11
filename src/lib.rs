use anyhow::{Context, Result};
use std::borrow::Cow;
use std::fmt;
use url::Url;

use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImUrl {
    url_str: Cow<'static, str>,
    url: Arc<Url>,
}

impl ImUrl {
    pub fn parse(input: &str) -> Result<Self> {
        let url_str = Cow::Owned(input.to_string());
        let url = Arc::new(Url::parse(input).context("Failed to parse URL")?);
        Ok(Self { url_str, url })
    }

    pub fn as_str(&self) -> &str {
        &self.url_str
    }

    pub fn scheme(&self) -> &str {
        self.url.scheme()
    }

    pub fn host(&self) -> Option<&str> {
        self.url.host_str()
    }

    pub fn port(&self) -> Option<u16> {
        self.url.port()
    }

    pub fn path(&self) -> &str {
        self.url.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.url.query()
    }

    pub fn fragment(&self) -> Option<&str> {
        self.url.fragment()
    }

    pub fn with_path(&self, path: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_path(path);
        Self::parse(url.as_str())
    }

    pub fn with_query(&self, query: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_query(Some(query));
        Self::parse(url.as_str())
    }

    pub fn with_fragment(&self, fragment: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_fragment(Some(fragment));
        Self::parse(url.as_str())
    }

    pub fn with_scheme(&self, scheme: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_scheme(scheme)
            .map_err(|_| anyhow::anyhow!("Failed to set scheme"))?;
        Self::parse(url.as_str())
    }

    pub fn with_username(&self, username: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_username(username).unwrap();
        Self::parse(url.as_str())
    }

    pub fn with_password(&self, password: Option<&str>) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_password(password).unwrap();
        Self::parse(url.as_str())
    }

    pub fn with_host(&self, host: &str) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_host(Some(host)).context("Failed to set host")?;
        Self::parse(url.as_str())
    }

    pub fn with_port(&self, port: u16) -> Result<Self> {
        let mut url = (*self.url).clone();
        url.set_port(Some(port))
            .map_err(|_| anyhow::anyhow!("Failed to set port"))?;
        Self::parse(url.as_str())
    }

    pub fn with_path_segments<I, S>(&self, segments: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut url = (*self.url).clone();
        url.path_segments_mut()
            .map_err(|_| anyhow::anyhow!("Cannot be a base URL"))?
            .clear()
            .extend(segments);
        Self::parse(url.as_str())
    }
}

impl fmt::Display for ImUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url_str)
    }
}
