#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() -> Result<()> {
        let url_str = "https://example.com/path?query=value#fragment";
        let im_url = ImUrl::parse(url_str)?;
        assert_eq!(im_url.as_str(), url_str);
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let url_str = "https://example.com/path?query=value#fragment";
        let im_url = ImUrl::parse(url_str)?;
        assert_eq!(im_url.to_string(), url_str);
        Ok(())
    }

    #[test]
    fn test_with_path_segments() -> Result<()> {
        let im_url = ImUrl::parse("https://example.com")?;
        let updated = im_url.with_path_segments(["api", "v1", "users"])?;
        assert_eq!(updated.as_str(), "https://example.com/api/v1/users");
        Ok(())
    }

    #[test]
    fn test_with_username() -> Result<()> {
        let im_url = ImUrl::parse("https://example.com")?;
        let updated = im_url.with_username("user")?;
        assert_eq!(updated.as_str(), "https://user@example.com");
        Ok(())
    }

    #[test]
    fn test_with_password() -> Result<()> {
        let im_url = ImUrl::parse("https://user@example.com")?;
        let updated = im_url.with_password(Some("pass"))?;
        assert_eq!(updated.as_str(), "https://user:pass@example.com");
        Ok(())
    }

    #[test]
    fn test_with_host() -> Result<()> {
        let im_url = ImUrl::parse("https://example.com")?;
        let updated = im_url.with_host("newhost.com")?;
        assert_eq!(updated.as_str(), "https://newhost.com");
        Ok(())
    }

    #[test]
    fn test_with_port() -> Result<()> {
        let im_url = ImUrl::parse("https://example.com")?;
        let updated = im_url.with_port(8080)?;
        assert_eq!(updated.as_str(), "https://example.com:8080");
        Ok(())
    }

    #[test]
    fn test_as_str() -> Result<()> {
        let url_str = "https://example.com/path?query=value#fragment";
        let im_url = ImUrl::parse(url_str)?;
        assert_eq!(im_url.as_str(), url_str);
        Ok(())
    }

    use anyhow::Result;
    use imurl::ImUrl;
    use url::Url;

    #[test]
    fn test_imurl_equality_with_standard_url() -> Result<()> {
        // Create a standard Url
        let mut standard_url = Url::parse("https://example.com").unwrap();
        standard_url.set_path("/new/path");
        standard_url.set_query(Some("key=value"));

        #[test]
        fn test_with_path() -> Result<()> {
            let im_url = ImUrl::parse("https://example.com")?;
            let updated = im_url.with_path("/new/path")?;
            assert_eq!(updated.as_str(), "https://example.com/new/path");
            Ok(())
        }

        #[test]
        fn test_with_query() -> Result<()> {
            let im_url = ImUrl::parse("https://example.com")?;
            let updated = im_url.with_query("key=value")?;
            assert_eq!(updated.as_str(), "https://example.com/?key=value");
            Ok(())
        }

        #[test]
        fn test_with_fragment() -> Result<()> {
            let im_url = ImUrl::parse("https://example.com")?;
            let updated = im_url.with_fragment("section")?;
            assert_eq!(updated.as_str(), "https://example.com/#section");
            Ok(())
        }

        #[test]
        fn test_invalid_url() {
            assert!(ImUrl::parse("not a url").is_err());
        }

        #[test]
        fn test_invalid_scheme() {
            let url = ImUrl::parse("https://example.com").unwrap();
            assert!(url.with_scheme("not-a-scheme").is_err());
        }

        #[test]
        fn test_cannot_be_a_base() {
            let url = ImUrl::parse("mailto:user@example.com").unwrap();
            assert!(url.with_path_segments(["test"]).is_err());
        }

        #[test]
        fn test_invalid_host() {
            let url = ImUrl::parse("https://example.com").unwrap();
            assert!(url.with_host("").is_err());
        }

        #[test]
        fn test_invalid_port() {
            let url = ImUrl::parse("https://example.com").unwrap();
            assert!(url.with_port(65536).is_err());
        }

        #[test]
        fn test_with_scheme() -> Result<()> {
            let im_url = ImUrl::parse("https://example.com")?;
            let updated = im_url.with_scheme("http")?;
            assert_eq!(updated.as_str(), "http://example.com");
            Ok(())
        }

        standard_url.set_fragment(Some("section"));
        standard_url.set_scheme("http").unwrap();
        standard_url.set_username("user").unwrap();
        standard_url.set_password(Some("pass")).unwrap();
        standard_url.set_host(Some("example.org")).unwrap();
        standard_url.set_port(Some(8080)).unwrap();
        let standard_url = standard_url.to_string();

        // Create an ImUrl with the same components
        let im_url = ImUrl::parse("https://example.com")?
            .with_path("/new/path")?
            .with_query("key=value")?
            .with_fragment("section")?
            .with_scheme("http")?
            .with_username("user")?
            .with_password(Some("pass"))?
            .with_host("example.org")?
            .with_port(8080)?
            .to_string();

        // Compare the string representations
        assert_eq!(standard_url, im_url);

        Ok(())
    }
}
