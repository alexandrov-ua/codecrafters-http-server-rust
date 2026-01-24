use std::collections::HashMap;

use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct UrlMatcher {
    pattern: String,
}

impl UrlMatcher {
    pub fn new(pattern: &str) -> Self {
        let re = Regex::new("^((\\/\\{[\\w\\-\\.]+\\})?(\\/[\\w\\-\\.]+)?)+(\\/\\{[\\w\\-]+\\*?\\})?\\/?$").unwrap();
        if re.is_match(pattern) == false {
            panic!("Invalid URL pattern: {}", pattern);
        }
        UrlMatcher{pattern: pattern.to_string()}
    }

    pub fn match_url(&self, url: &str) -> (bool, HashMap<String, String>) {
        let mut params: HashMap<String, String> = HashMap::new();
        let pattern_parts: Vec<&str> = self.pattern.split('/').collect();
        let url_parts: Vec<&str> = url.split('/').collect();

        let mut i = 0;
        while i < pattern_parts.len() {
            let pattern_part = pattern_parts[i];
            if pattern_part.starts_with('{') && pattern_part.ends_with('}') {
                let param_name = &pattern_part[1..pattern_part.len()-1];
                if param_name.ends_with('*') {
                    let param_name = &param_name[..param_name.len()-1];
                    let remaining_url = url_parts[i..].join("/");
                    params.insert(param_name.to_string(), remaining_url);
                    return (true, params);
                } else {
                    if i >= url_parts.len() {
                        return (false, HashMap::new());
                    }
                    params.insert(param_name.to_string(), url_parts[i].to_string());
                }
            } else {
                if i >= url_parts.len() || pattern_part != url_parts[i] {
                    return (false, HashMap::new());
                }
            }
            i += 1;
        }

        if i != url_parts.len() {
            return (false, HashMap::new());
        }

        (true, params)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_not_panic() {
        let _ = UrlMatcher::new("/users/{userId}/posts/{postId}");
        let _ = UrlMatcher::new("/users/{userId}/posts/{postId}/");
        let _ = UrlMatcher::new("/users/posts/");
        let _ = UrlMatcher::new("/users/posts");
        let _ = UrlMatcher::new("/users/{userId}/posts/{postId*}");
        let _ = UrlMatcher::new("/users/{userId}/posts/{postId*}/");
        let _ = UrlMatcher::new("/");
    }

    #[test]
    #[should_panic]
    fn new_should_panic_on_invalid_pattern() {
        let _ = UrlMatcher::new("users/{userId");
    }

    #[test]
    fn match_url_should_work() {
        let matcher = UrlMatcher::new("/users/{userId}/posts/{postId}");
        let (matched, params) = matcher.match_url("/users/123/posts/456");
        assert!(matched);
        assert_eq!(params.get("userId").unwrap(), "123");
        assert_eq!(params.get("postId").unwrap(), "456");
    }

    #[test]
    fn match_url_should_work_wildcard() {
        let matcher = UrlMatcher::new("/users/{wildcard*}");
        let (matched, params) = matcher.match_url("/users/123/posts/456");
        assert!(matched);
        assert_eq!(params.get("wildcard").unwrap(), "123/posts/456");
    }

    #[test]
    fn match_url_should_work_ending_slash() {
        let matcher = UrlMatcher::new("/users/");
        let (matched, params) = matcher.match_url("/users/");
        assert!(matched);
        assert_eq!(params.len(), 0);
    }
}
