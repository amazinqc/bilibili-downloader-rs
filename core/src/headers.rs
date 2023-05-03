//! A header generation

use crate::config::USER_AGENT;
use reqwest::header;

const MINI_SIZE: usize = 5000000;

#[derive(Debug)]
pub struct Headers {
    start: usize,
    to: usize,
    end: usize,
}

fn headers(range: String) -> header::HeaderMap {
    let headers_json = format!(
        r#"
{{
    "Sec-Fetch-Site": "cross-site",
    "Accept-Language": "zh-CN,zh-Hans;q=0.9",
    "Accept-Encoding": "identity",
    "Sec-Fetch-Mode": "cors",
    "Origin": "https://www.bilibili.com",
    "User-Agent": "{USER_AGENT}",
    "Referer": "https://www.bilibili.com/",
    "Range": "bytes={range}",
    "Sec-Fetch-Dest": "empty"
}}"#
    );
    let hm: std::collections::HashMap<String, String> =
        serde_json::from_str(&headers_json).unwrap();
    let headers: header::HeaderMap = (&hm).try_into().unwrap();
    headers
}

impl Headers {
    // input a range
    // return an iterator contains several headers
    pub fn new(range: String) -> Self {
        let mut temp = range.split('-');
        let start = temp.next().unwrap().parse::<usize>().unwrap();
        let end = temp.next().unwrap().parse::<usize>().unwrap();
        Self {
            start,
            to: std::cmp::min(start + MINI_SIZE, end),
            end,
        }
    }
}

impl Iterator for Headers {
    type Item = header::HeaderMap;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let range = format!("{}-{}", self.start, self.to);
        let hm = headers(range);

        self.start = self.to + 1;
        self.to = std::cmp::min(self.to + MINI_SIZE, self.end);
        Some(hm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let headers = Headers::new(String::from("1-8889877"));
        for i in headers {
            dbg!(i);
        }
    }
}