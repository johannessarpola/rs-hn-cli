use std::io;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::collections::VecDeque;

use hyper::client::HttpConnector;
use hyper::{Client};
use native_tls::TlsConnector;
use tokio_core::reactor::{Core, Handle};

use slog;
use slog_term;
use slog_stream;
use slog::{Level, LevelFilter, DrainExt};
use super::endpoint::HnNewsEndpoint;
use super::models::{HnItem, HnListOfItems};
use super::connector::HttpsConnector;
use utils::comment_has_kids;

///
/// 'AppDomain' struct which have relevant parts which are use as core elements of the application
///
pub struct AppDomain {
    pub core: Core,
    pub endpoint: HnNewsEndpoint,
    pub client: Client<HttpsConnector>,
    pub logger: slog::Logger,
}

pub struct AppCache {
    pub retrieved_top_stories: Option<HnListOfItems>,
    pub retrieved_best_stories: Option<HnListOfItems>,
    pub retrieved_new_stories: Option<HnListOfItems>,
    pub last_retrieved_item: Option<HnItem>,
    pub last_parent_items: VecDeque<HnItem>, // this does not need to be optional
    pub last_retrieved_comments: Option<Vec<HnItem>>,
}

impl AppCache {
    pub fn new() -> AppCache {
        AppCache {
            retrieved_top_stories: None,
            retrieved_best_stories: None,
            retrieved_new_stories: None,
            last_retrieved_item: None,
            last_parent_items: VecDeque::new(),
            last_retrieved_comments: None,
        }
    }
    pub fn get_comment(&mut self, numb: usize) -> Option<HnItem> {
        match self.last_retrieved_comments {
            Some(ref mut comments) => Some(comments.remove(numb)),
            None => None,
        }
    }

    pub fn get_comment_if_kids(&mut self, numb: usize) -> Option<HnItem> {
        match self.last_retrieved_comments {
            Some(ref mut comments) => {
                if comment_has_kids(&comments[numb]) {
                    Some(comments.remove(numb))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn stories_len(&self) -> Option<usize> {
        match self.retrieved_top_stories {
            Some(ref top_stories) => Some(top_stories.values.len()),
            None => None,
        }
    }

    pub fn comments_len(&self) -> Option<usize> {
        match self.last_retrieved_comments {
            Some(ref comments) => Some(comments.len()),
            None => None,
        }
    }
}

pub enum AppStates {
    WaitingUserInput,
    RetrievingResults,
    DoingLocalWork,
    Idle,
    Starting,
}

pub struct AppStateMachine {
    pub viewing_top_stories: bool,
    pub viewing_comments_for_a_story: bool,
    pub connection_working: bool,
    pub listing_page_index: i32,
    pub comments_page_index: i32,
    pub last_opened_item_id: String,
    pub current_state: AppStates,
}

impl AppStateMachine {
    pub fn new() -> AppStateMachine {
        AppStateMachine {
            viewing_top_stories: false,
            viewing_comments_for_a_story: false,
            connection_working: false,
            listing_page_index: 0,
            comments_page_index: 0,
            last_opened_item_id: String::from(""),
            current_state: AppStates::Starting,
        }
    }
}

struct AppLogFormat;

impl slog_stream::Format for AppLogFormat {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &slog::Record,
              _logger_values: &slog::OwnedKeyValueList)
              -> io::Result<()> {
        let msg = format!("{} - {}\n", rinfo.level(), rinfo.msg());
        let _ = try!(io.write_all(msg.as_bytes()));
        Ok(())
    }
}

fn configure_client(handle: &Handle) -> Client<HttpsConnector> {
    let tls_cx = TlsConnector::builder().unwrap().build().unwrap();
    let mut connector = HttpsConnector {
        tls: Arc::new(tls_cx),
        http: HttpConnector::new(4, handle),
    };
    connector.disable_enforce_http();
    Client::configure()
            .connector(connector)
            .build(handle)

}

impl AppDomain {
    pub fn new() -> AppDomain {
        let logger = create_loggers();
        let core = Core::new().expect("Failed to create core");
        let handle = core.handle();
        let client = configure_client(&handle);
        let endpoint = HnNewsEndpoint::build_default();
        AppDomain {
            core: core,
            endpoint: endpoint,
            client: client,
            logger: logger,
        }
    }
}

fn create_loggers() -> slog::Logger {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .open("app.log")
        .unwrap();
    let file_drain = slog_stream::stream(file, AppLogFormat);
    let std_out_drain = slog_term::streamer().build();
    // let logger = slog::Logger::root(slog::duplicate(console_drain, file_drain).fuse(), o!());
    let logger =
        slog::Logger::root(slog::Duplicate::new(LevelFilter::new(file_drain, Level::Info),
                                                LevelFilter::new(std_out_drain, Level::Warning))
                               .fuse(),
                           o!());
    logger
}