use hyper::{Uri};
use core::models::HnItem;

pub fn parse_url_from_str(url_str: &str) -> Uri {
    let url_str = String::from(url_str);
    let url = url_str.parse::<Uri>().unwrap();
    url
}

pub fn generate_filename_for_hnitem(item: &HnItem) -> String {
    match item.title {
        Some(ref title) => return combine_strings(vec![&title, &item.by, ".html"]),
        None => {
            return combine_strings(vec![&parse_url_from_str(&item.url.as_ref().unwrap())
                                            .path()
                                            .replace("/", "_"),
                                        ".html"])
        }
    }
}

fn combine_strings(strings: Vec<&str>) -> String {
    let combine = strings.join("");
    combine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_from_str_test() {
        let url = parse_url_from_str("http://www.google.fi");
        assert_eq!("http", url.scheme().unwrap());
        assert_eq!("www.google.fi", url.authority().unwrap());
    }
}