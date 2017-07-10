use core::models::*;
use super::colors;

pub fn print_comment_and_parent(item: &Option<&HnItem>, comments: &Option<Vec<HnItem>>) {
    match *item {
        Some(ref item) => {
            match *comments {
                Some(ref comments) => print_comments(item, comments),
                None => could_not_get_any_commments_for_item(item), 
            }
        }
        None => (),
    }
}

pub fn print_headline_with_author(item: &HnItem, index: &i32) {
    let s = create_headline_with_author(item, index).unwrap(); // Not handling errs
    println!("{}", s);
}

fn create_headline_with_author(item: &HnItem, index: &i32) -> Result<String, String> {
    match item.title {
        Some(_) => {
            let s = format!("[{:3}] {:80} ~ {}",
                            index,
                            item.title.as_ref().unwrap(),
                            item.by);
            Ok(s)

        }
        None => Err(String::from("Not headline")),
    }
}

pub fn could_not_get_any_commments_for_item(item: &HnItem) {
    println!("Could not get comments for item with id {}", item.id)
}

pub fn print_filename_of_loaded_page(filen: &str, title: &str) {
    println!("{} {} {} {}", "Downloaded page", title, "into file", filen);
}
pub fn could_not_load_page(title: &str) {
    println!("Could not download to file with title {}", title);
}

pub fn print_no_connection() {
    println!("Could not detect internet connection, please check it and try again");
}

pub fn print_comments(item: &HnItem, comments: &Vec<HnItem>) {
    let coloring = colors::CliColoring::new(colors::Theme::Disabled);  // todo, probably from app_domain

    if comments.len() > 0 {
        match item.title {
            Some(ref title) => println!("Comments for item id {} with title {}", &item.id, title),
            None => println!("Comments for item id {}", &item.id),
        }
        let mut comment_index = 0;
        for comment in comments {
            comment_index += 1;
            let res = create_comment_row(&comment_index, &comment);
            if res.is_some() {
                if comment_index % 2 == 0 {
                    coloring.zebra_coloring();
                }

                println!("{}", res.unwrap());
                coloring.reset_colors();
            } else {
                comment_index -= 1;
            }
        }
    } else {
        println!("No comments for {}", item.id);
    }
    println!("") // this adds extra space after comments and prevent the input being interfered with bg coloring
}

pub fn print_invalid_numb() {
    println!("Received invalid number");
}

pub fn print_over_limit_but_using_index(numb:usize) {
    println!("Over the limit, using index {}", numb);
}

pub fn print_could_not_get_story(numb:usize) {
    println!("Could not get story at index {}", numb);
}

fn create_comment_row(index: &i32, item: &HnItem) -> Option<String> {
    match item.text_unescaped() {
        Some(ref text) => {
            let mut s = format!("[{:3}] {:80} ~{}", index, text, &item.by); // TODO This needs to handle unicode characters to utf8 or something similar (snap&#x27;s -> snap's)
            match item.kids {
                Some(ref kids) => s.push_str(&format!(" with [{:3}] replies", kids.len())),
                None => ()
            }
            s.push_str("\n-----");
            Some(s)
        }
        None => None,
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_headline_with_author_test() {
        use std::fs::File;
        use std::io::prelude::*;
        use serde_json;
        let mut contents = String::new();
        File::open("res/test/item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let index = 1;
        let s: String = create_headline_with_author(&deserialized, &index).unwrap();
        assert!(s.len() != 0);
        assert!(s.contains("1"));
        assert!(s.contains("dhouston"));
        assert!(s.contains("My YC app: Dropbox - Throw away your USB drive"));
        assert!(deserialized.by.len() != 0);
        assert!(deserialized.title.unwrap().len() != 0);
    }

    #[test]
    fn create_comment_row_test() {
        use std::fs::File;
        use std::io::prelude::*;
        use serde_json;
        let mut contents = String::new();
        File::open("res/test/children-item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let commentStr = create_comment_row(&1, &deserialized).unwrap();
        assert!(commentStr.contains("is not a valid concern. Unless you are planning"));
        assert!(commentStr.contains("cholantesh"));

    }

    #[test]
    fn test_cli_color() {}
}