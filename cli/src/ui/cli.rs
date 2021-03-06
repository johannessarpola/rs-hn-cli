use core::models::*;
use formatting::formatter::FormatStr;
use helpers::path_utils;

const HELP_STR: &'static str  = "
top             > opens the currently opened page of stories (reprints)
next            > retrieves the next 10 stories or comments
back            > retrieves the previous 10 stories or comments
comments [num]  > retrieves comments for given story, based on the id of the story shown in [num] ten at a time
expand [num]    > once comments are open you can retrieve the sub comments for the comment with it ten at a time
load [num]      > loads the page linked in the story as local html
open [num]      > opens the link with default browser
exit            > quits the application

[num] replace the number with the printed out index";

pub fn print_help() {
    println!("{}", HELP_STR);
}

pub fn print_tried_to_navigate_over_index() {
    println!("Tried to access next or back over the index");
}

pub fn print_comments_and_parent(item: Option<&HnItem>,
                                comments: &Option<Vec<&HnItem>>,
                                format: &FormatStr, 
                                index:usize) {
    match item {
        Some(ref item) => {
            match *comments {
                Some(ref comments) => print_comments(item, comments, format, index),
                None => could_not_get_any_commments_for_item(item), 
            }
        }
        None => (),
    }
}

pub fn print_invalid_command() {
    println!("Could not understand command, please try again or check help");
}

pub fn print_headline_with_author(item: &HnItem, index: &i32) {
    let s = create_headline_with_author(item, index).unwrap(); // Not handling errs
    println!("{}", s);
}

fn create_headline_with_author(item: &HnItem, index: &i32) -> Result<String, String> {
    let link = item.url.as_ref().and_then(|link| path_utils::get_host_from_link(link)).unwrap_or("could not parse link".to_owned());
    match item.title {
        Some(_) => {
            let headline_with_link = format!("{} ({})",item.title.as_ref().unwrap(), link);
            let s = format!("[{:3}] {:70} by {} with [{}] comments",
                            index,
                            headline_with_link,
                            item.by,
                            item.kids.as_ref().unwrap_or(&Vec::new()).len());
            Ok(s)

        }
        None => Err(String::from("Not headline")),
    }
}

pub fn print_warning_for_downloading_page() {
    println!("Be careful when opening downloaded files, as this will just call curl on the page url without any checks for content");
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

pub fn print_invalid_state() {
    println!("App has an invalid state, could cause problems.");
}

pub fn print_comments(item: &HnItem, comments: &Vec<&HnItem>, format: &FormatStr, index:usize) {
    if comments.len() > 0 {
        match item.title {
            Some(ref title) => println!("Comments for item id {} with title {}", &item.id, title),
            None => println!("Comments for item id {}", &item.id),
        }
        let mut comment_index = index;
        for comment in comments {
            comment_index += 1;
            let res = create_comment_row(comment_index, &comment, format);
            if res.is_some() {
                println!("{}", res.unwrap());
            } else {
                comment_index -= 1;
            }
        }
    } else {
        println!("No comments for {} or all were dead (probably spam)",
                 item.id);
    }
}

pub fn print_no_comments_for(numb: usize) {
    println!("No comments for {}", numb);
}

pub fn print_invalid_numb() {
    println!("Received invalid number");
}

pub fn print_over_limit_but_using_index(numb: usize) {
    println!("Over the limit, using index {}", numb);
}

pub fn print_could_not_get_story(numb: usize) {
    println!("Could not get story at index {}", numb);
}

fn create_comment_row(index: usize, item: &HnItem, format: &FormatStr) -> Option<String> {
    match item.text_unescaped() {
        Some(ref text) => {
            let mut s = format!("[{:3}] {:70} by {}", index, &format.format(text), &item.by);
            match item.kids {
                Some(ref kids) => s.push_str(&format!(" with [{:3}] comments", kids.len())),
                None => (),
            }
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
        use formatting::formatter::Formatters;

        let mut contents = String::new();
        let formatting = Formatters::new();
        File::open("res/test/children-item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        let comment_str = create_comment_row(1, &deserialized, &formatting).unwrap();
        assert!(comment_str.contains("is not a valid concern. Unless you are planning"));
        assert!(comment_str.contains("cholantesh"));

    }
    
}