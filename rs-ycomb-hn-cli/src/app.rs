use tokio_core::reactor::{Core, Handle};
use hyper::Client;
use hyper_tls::HttpsConnector;
use endpoint::HnNewsEndpoint;
use slog::*;
use slog_term::*;

///
/// 'AppDomain' struct which have relevant parts which are use as core elements of the application
///
pub struct AppDomain {
    pub core: Core,
    pub endpoint: HnNewsEndpoint,
    pub client: Client<HttpsConnector>,
    pub logger: Logger,
}

fn configure_client(handle: &Handle) -> Client<HttpsConnector> {
    Client::configure()
            // Does not check the validity of certificate
            .connector(HttpsConnector::new(4, &handle))
            .build(&handle)
}

pub fn create_app_domain() -> AppDomain {
    let logger = create_loggers();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = configure_client(&handle);
    let endpoint = HnNewsEndpoint::build_default();
    let mut app_domain = AppDomain {
        core: core,
        endpoint: endpoint,
        client: client,
        logger: logger,
    };
    app_domain
}
fn create_loggers() -> Logger {
    let drain = streamer().build().fuse();
    let root_logger = Logger::root(drain, o!());
    root_logger
}
