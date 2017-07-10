use std::io;
use std::sync::Arc;

use futures::future::{err, Future};
use hyper::client::HttpConnector;
use hyper::{Uri};
use native_tls::TlsConnector;
use tokio_core::net::TcpStream;
use tokio_service::Service;
use tokio_tls::{TlsConnectorExt, TlsStream};


pub struct HttpsConnector {
    pub tls: Arc<TlsConnector>,
    pub http: HttpConnector,
}

impl HttpsConnector {
    pub fn disable_enforce_http(&mut self) {
        self.http.enforce_http(false);
    }
}

impl Service for HttpsConnector {
    type Request = Uri;
    type Response = TlsStream<TcpStream>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, uri: Uri) -> Self::Future {
        // Right now this is intended to showcase `https`, but you could
        // also adapt this to return something like `MaybeTls<T>` where
        // some clients resolve to TLS streams (https) and others resolve
        // to normal TCP streams (http)
        if uri.scheme() != Some("https") {
            return err(io::Error::new(io::ErrorKind::Other,
                                      "only works with https")).boxed()
        }

        // Look up the host that we're connecting to as we're going to validate
        // this as part of the TLS handshake.
        let host = match uri.host() {
            Some(s) => s.to_string(),
            None =>  {
                return err(io::Error::new(io::ErrorKind::Other,
                                          "missing host")).boxed()
            }
        };

        // Delegate to the standard `HttpConnector` type to create a connected
        // TCP socket. Once we've got that socket initiate the TLS handshake
        // with the host name that's provided in the URI we extracted above.
        let tls_cx = self.tls.clone();
        Box::new(self.http.call(uri).and_then(move |tcp| {
            tls_cx.connect_async(&host, tcp)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }))
    }
}