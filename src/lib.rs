/// Implements struct hierarchy and serializer for the [HAR 1.2 spec][1].
///
/// [1]: http://www.softwareishard.com/blog/har-12-spec/

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

const HAR_VERSION: &'static str = "1.2";
const HAR_CREATOR_NAME: &'static str = "Rust-HAR";
const HAR_CREATOR_VERSION: &'static str = "0.0.4";

/// This object represents the root of the exported data.
///
/// This object MUST be present and its name MUST be "log".
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    /// Version number of the format.
    version: String,

    /// Name and version info of the log creator application.
    creator: Creator,

    /// Name and version info of used browser.
    browser: Option<Browser>,

    /// List of all exported (tracked) pages.
    /// Leave out this field if the application does not support grouping by pages.
    pages: Option<Vec<Page>>,

    /// List of all exported (tracked) requests.
    entries: Vec<Entry>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Log {
    pub fn new(browser: Option<Browser>, comment: Option<String>) -> Log {
        Log {
            version: HAR_VERSION.to_string(),
            creator: Creator::new(
                HAR_CREATOR_NAME.to_string(),
                HAR_CREATOR_VERSION.to_string(),
                None
            ),
            browser: browser,
            pages: None,
            entries: Vec::new(),
            comment: comment
        }
    }

    pub fn add_page(&mut self, page: Page) {
        match self.pages {
            Some(ref mut pages) => pages.push(page),
            None => self.pages = Some(vec![page])
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

/// This object contains information about the log creator application.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    name: String,
    version: String,
    comment: Option<String>
}

impl Creator {
    pub fn new(name: String, version: String, comment: Option<String>) -> Creator{
        Creator {
            name: name,
            version: version,
            comment: comment
        }
    }
}

/// This object contains information about the browser that created the log.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    name: String,
    version: String,
    comment: Option<String>
}

impl Browser {
    pub fn new(name: String, version: String, comment: Option<String>) -> Browser{
        Browser {
            name: name,
            version: version,
            comment: comment
        }
    }
}

/// This object represents list of exported pages.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// Date and time stamp for the beginning of the page load
    /// (ISO 8601 YYYY-MM-DDThh:mm:ss.sTZD, e.g. 2009-07-24T19:20:30.45+01:00).
    started_date_time: String,
    /// Unique identifier of a page within the . Entries use it to refer the parent page.
    id: String,
    /// Page title.
    title: String,
    /// Detailed timing info about page load.
    page_timings: PageTimings,
    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Page {
    pub fn new(started_date_time: String,
               id: String,
               title: String,
               page_timings: PageTimings,
               comment: Option<String>) -> Page {
        Page {
            started_date_time: started_date_time,
            id: id,
            title: title,
            page_timings: page_timings,
            comment: comment
        }
    }
}

/// This object describes timings for various events (states) fired during the page load.
///
/// All times are specified in milliseconds.
/// If a time info is not available appropriate field is set to -1.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageTimings {
    /// Content of the page loaded.
    /// Number of milliseconds since page load started (page.startedDateTime).
    /// Use -1 if the timing does not apply to the current request.
    on_content_load: OptionalTiming,

    /// Page is loaded (onLoad event fired).
    /// Number of milliseconds since page load started (page.startedDateTime).
    /// Use -1 if the timing does not apply to the current request.
    on_load: OptionalTiming,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl PageTimings {
    pub fn new(
        on_content_load: OptionalTiming,
        on_load: OptionalTiming,
        comment: Option<String>
    ) -> PageTimings {
        PageTimings {
            on_content_load: on_content_load,
            on_load: on_load,
            comment: comment,
        }
    }
}

/// This object represents an array with all exported HTTP requests. Sorting entries by
/// startedDateTime (starting from the oldest) is preferred way how to export data since it can
/// make importing faster. However the reader application should always make sure the array is
/// sorted (if required for the import).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Reference to the parent page (unique).
    /// Leave out this field if the application does not support grouping by pages.
    pageref: Option<String>,

    /// Date and time stamp of the request start (ISO 8601 YYYY-MM-DDThh:mm:ss.sTZD).
    started_date_time: String,

    /// Total elapsed time of the request in milliseconds.
    /// This is the sum of all timings available in the timings object.
    // time [number]

    /// Detailed info about the request.
    request: Request,

    /// Detailed info about the response.
    response: Response,

    /// Info about cache usage.
    cache: Cache,

    /// Detailed timing info about request/response round trip.
    timings: Timing,

    /// IP address of the server that was connected (result of DNS resolution).
    server_ip_address: Option<String>,

    /// Unique ID of the parent TCP/IP connection, can be the client port number.
    ///
    /// Note that a port number doesn't have to be unique identifier in cases where the port is
    /// shared for more connections. If the port isn't available for the application, any other
    /// unique connection ID can be used instead (e.g. connection index). Leave out this field if
    /// the application doesn't support this info.
    connection: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Entry {
    pub fn new(
        pageref: Option<String>,
        started_date_time: String,
        request: Request,
        response: Response,
        cache: Cache,
        timings: Timing,
        server_ip_address: Option<String>,
        connection: Option<String>,
        comment: Option<String>
    ) -> Entry {
        Entry {
            pageref: pageref,
            started_date_time: started_date_time,
            request: request,
            response: response,
            cache: cache,
            timings: timings,
            server_ip_address: server_ip_address,
            connection: connection,
            comment: comment
        }
    }
}

/// This object contains detailed info about performed request.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Request method (GET, POST, ...).
    method: String,

    /// Absolute URL of the request (fragments are not included).
    url: String,

    /// Request HTTP Version.
    http_version: String,

    /// List of cookie objects.
    cookies: Vec<Cookie>,

    /// List of header objects.
    headers: Vec<Header>,

    /// List of query parameter objects.
    query_string: Vec<QueryStringPair>,

    /// Posted data info.
    post_data: Option<PostData>,

    /// Total number of bytes from the start of the HTTP request message until (and including)
    /// the double CRLF before the body.
    /// Set to -1 if the info is not available.
    headers_size: Option<i32>,

    /// Size of the request body (POST data payload) in bytes.
    /// Set to -1 if the info is not available.
    body_size: Option<i32>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Request {
    pub fn new(
        method: String,
        url: String,
        http_version: String,
        cookies: Vec<Cookie>,
        headers: Vec<Header>,
        query_string: Vec<QueryStringPair>,
        post_data: Option<PostData>,
        headers_size: Option<i32>,
        body_size: Option<i32>,
        comment: Option<String>
    ) -> Request {
        Request {
            method: method,
            url: url,
            http_version: http_version,
            cookies: cookies,
            headers: headers,
            query_string: query_string,
            post_data: post_data,
            headers_size: headers_size,
            body_size: body_size,
            comment: comment
        }
    }
}

/// This object contains detailed info about the response.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Response status.
    status: i32,

    /// Response status description.
    status_text: String,

    /// Response HTTP Version.
    http_version: String,

    /// List of cookie objects.
    cookies: Vec<Cookie>,

    /// List of header objects.
    headers: Vec<Header>,

    /// Details about the response body.
    content: Content,

    /// Redirection target URL from the Location response header.
    redirect_url: String,

    /// Total number of bytes from the start of the HTTP response message until (and including) the
    /// double CRLF before the body.
    /// Set to -1 if the info is not available.
    /// The size of received response-headers is computed only from headers that are really
    /// received from the server. Additional headers appended by the browser are not included in
    /// this number, but they appear in the list of header objects.
    headers_size: Option<i32>,

    /// Size of the received response body in bytes.
    /// Set to zero in case of responses coming from the cache (304).
    /// Set to -1 if the info is not available.
    body_size: Option<i32>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Response {
    pub fn new(
        status: i32,
        status_text: String,
        http_version: String,
        cookies: Vec<Cookie>,
        headers: Vec<Header>,
        content: Content,
        redirect_url: String,
        headers_size: Option<i32>,
        body_size: Option<i32>,
        comment: Option<String>
    ) -> Response {
        Response {
            status: status,
            status_text: status_text,
            http_version: http_version,
            cookies: cookies,
            headers: headers,
            content: content,
            redirect_url: redirect_url,
            headers_size: headers_size,
            body_size: body_size,
            comment: comment
        }
    }
}


/// This object contains list of all cookies (used in <request> and <response> objects).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// The name of the cookie.
    name: String,

    /// The cookie value.
    value: String,

    /// The path pertaining to the cookie.
    path: Option<String>,

    /// The host of the cookie.
    domain: Option<String>,

    /// Cookie expiration time. (ISO 8601).
    expires: Option<String>,

    /// Set to true if the cookie is HTTP only, false otherwise.
    http_only: Option<bool>,

    /// True if the cookie was transmitted over ssl, false otherwise.
    secure: Option<bool>,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl Cookie {
    pub fn new(
        name: String,
        value: String,
        path: Option<String>,
        domain: Option<String>,
        expires: Option<String>,
        http_only: Option<bool>,
        secure: Option<bool>,
        comment: Option<String>
    ) -> Cookie {
        Cookie {
            name: name,
            value: value,
            path: path,
            domain: domain,
            expires: expires,
            http_only: http_only,
            secure: secure,
            comment: comment
        }
    }
}


/// This object contains list of all headers (used in <request> and <response> objects).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    name: String,
    value: String,
    comment: Option<String>
}

impl Header {
    pub fn new(
        name: String,
        value: String,
        comment: Option<String>
    ) -> Header {
        Header {
            name: name,
            value: value,
            comment: comment
        }
    }
}

/// This object contains list of all parameters & values parsed from a query string, if any
/// (embedded in <request> object).
/// HAR format expects NVP (name-value pairs) formatting of the query string.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryStringPair {
    name: String,
    value: String,
    comment: Option<String>
}

impl QueryStringPair {
    pub fn new(
        name: String,
        value: String,
        comment: Option<String>
    ) -> QueryStringPair {
        QueryStringPair {
            name: name,
            value: value,
            comment: comment
        }
    }
}

/// This object describes posted data, if any (embedded in <request> object).
/// Note that text and params fields are mutually exclusive.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    /// Mime type of posted data.
    mime_type: String,

    /// List of posted parameters (in case of URL encoded parameters).
    params: Vec<Param>,

    /// Plain text posted data
    text: String,

    /// A comment provided by the user or the application.
    comment: Option<String>
}

impl PostData {
    pub fn new(
        mime_type: String,
        params: Vec<Param>,
        text: String,
        comment: Option<String>
    ) -> PostData {
        PostData {
            mime_type: mime_type,
            params: params,
            text: text,
            comment: comment
        }
    }
}

/// List of posted parameters, if any (embedded in <postData> object).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    /// name of a posted parameter.
    name: String,

    /// value of a posted parameter or content of a posted file.
    value: Option<String>,

    /// name of a posted file.
    file_name: Option<String>,

    /// content type of a posted file.
    content_type: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>,
}

impl Param {
    pub fn new(
        name: String,
        value: Option<String>,
        file_name: Option<String>,
        content_type: Option<String>,
        comment: Option<String>
    ) -> Param {
        Param {
            name: name,
            value: value,
            file_name: file_name,
            content_type: content_type,
            comment: comment
        }
    }
}

/// This object describes details about response content (embedded in <response> object).
///
/// Before setting the text field, the HTTP response is decoded (decompressed & unchunked), than
/// trans-coded from its original character set into UTF-8. Additionally, it can be encoded using
/// e.g. base64. Ideally, the application should be able to unencode a base64 blob and get a
/// byte-for-byte identical resource to what the browser operated on.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    /// Length of the returned content in bytes.
    /// Should be equal to response.bodySize if there is no compression and bigger when the content
    /// has been compressed.
    size: i32,

    /// Number of bytes saved. Leave out this field if the information is not available.
    compression: Option<i32>,

    /// MIME type of the response text (value of the Content-Type response header).
    /// The charset attribute of the MIME type is included (if available).
    mime_type: String,

    /// Response body sent from the server or loaded from the browser cache.
    /// This field is populated with textual content only.
    /// The text field is either HTTP decoded text or a encoded (e.g. "base64") representation of
    /// the response body.
    /// Leave out this field if the information is not available.
    text: Option<String>,

    /// Encoding used for response text field e.g "base64".
    /// Leave out this field if the text field is HTTP decoded (decompressed & unchunked),
    /// than trans-coded from its original character set into UTF-8.
    encoding: Option<String>,

    /// A comment provided by the user or the application.
    comment: Option<String>,
}

impl Content {
    pub fn new(
        size: i32,
        compression: Option<i32>,
        mime_type: String,
        text: Option<String>,
        encoding: Option<String>,
        comment: Option<String>
    ) -> Content {
        Content {
            size: size,
            compression: compression,
            mime_type: mime_type,
            text: text,
            encoding: encoding,
            comment: comment
        }
    }
}

/// This objects contains info about a request coming from browser cache.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    /// State of a cache entry before the request.
    /// Leave out this field if the information is not available.
    before_request: CacheState,

    /// State of a cache entry after the request.
    /// Leave out this field if the information is not available.
    after_request: CacheState,

    comment: Option<String>
}

impl Cache {
    pub fn new(
        before_request: CacheState,
        after_request: CacheState,
        comment: Option<String>
    ) -> Cache {
        Cache {
            before_request: before_request,
            after_request: after_request,
            comment: comment
        }
    }
}

/// The state of a cache entry.
///
/// Can be Absent, Present, or Unknown. When serialized, these result in (respectively) `null`, a
/// CacheEntry value, or no object.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum CacheState {
    Absent,
    Present(CacheEntry),
    Unknown
}

/*
use serde::de::{Deserialize, Deserializer, Visitor, Error};

impl Deserialize for CacheState {
    fn deserialize<D>(deserializer: D) -> Result<CacheState, D::Error>
        where D: Deserializer
    {
        struct FieldVisitor;

        impl Visitor for FieldVisitor {
            type Value = CacheState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("empty, null or cache entry")
            }

            fn visit_str<E>(self, value: &str) -> Result<CacheState, E>
                where E: Error
            {
                match value {
                    "" => Ok(CacheState::Absent),
                    "null" => Ok(CacheState::Unknown),
                    _ => Err(E::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.deserialize_struct_field(FieldVisitor)
    }
}
*/

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    /// Expiration time of the cache entry.
    expires: Option<String>,

    /// The last time the cache entry was opened.
    last_access: String,

    /// Etag
    e_tag: String,

    /// The number of times the cache entry has been opened.
    hit_count: i32,

    /// (new in 1.2) A comment provided by the user or the application.
    comment: Option<String>,
}

impl CacheEntry {
    pub fn new(
        expires: Option<String>,
        last_access: String,
        e_tag: String,
        hit_count: i32,
        comment: Option<String>
    ) -> CacheEntry {
        CacheEntry {
            expires: expires,
            last_access: last_access,
            e_tag: e_tag,
            hit_count: hit_count,
            comment: comment
        }
    }
}

/// A timing value which may be absent or present
///
/// Defaults to -1 in the absent case.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OptionalTiming {
    TimedContent(u32),
    NotApplicable
}

/// This object describes various phases within request-response round trip. All times are
/// specified in milliseconds.
///
/// The send, wait and receive timings are not optional and must have non-negative values.
///
/// An exporting tool can omit the blocked, dns, connect and ssl, timings on every request if it is
/// unable to provide them. Tools that can provide these timings can set their values to -1 if they
/// don’t apply. For example, connect would be -1 for requests which re-use an existing connection.
///
/// The time value for the request must be equal to the sum of the timings supplied in this section
/// (excluding any -1 values).
///
/// Following must be true in case there are no -1 values (entry is an object in log.entries) :
/// entry.time == entry.timings.blocked + entry.timings.dns +
///     entry.timings.connect + entry.timings.send + entry.timings.wait +
///         entry.timings.receive;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    /// Time spent in a queue waiting for a network connection.
    /// Use -1 if the timing does not apply to the current request.
    blocked: OptionalTiming,

    /// DNS resolution time. The time required to resolve a host name.
    /// Use -1 if the timing does not apply to the current request.
    dns: OptionalTiming,

    /// Time required to create TCP connection.
    /// Use -1 if the timing does not apply to the current request.
    connect: OptionalTiming,

    /// Time required to send HTTP request to the server.
    send: u32,

    /// Waiting for a response from the server.
    wait: u32,

    /// Time required to read entire response from the server (or cache).
    receive: u32,

    /// Time required for SSL/TLS negotiation.
    /// If this field is defined then the time is also included in the connect field (to ensure
    /// backward compatibility with HAR 1.1).
    /// Use -1 if the timing does not apply to the current request.
    ssl: OptionalTiming,

    /// (new in 1.2) - A comment provided by the user or the application.
    comment: Option<String>
}

impl Timing {
    pub fn new(
        blocked: OptionalTiming,
        dns: OptionalTiming,
        connect: OptionalTiming,
        send: u32,
        wait: u32,
        receive: u32,
        ssl: OptionalTiming,
        comment: Option<String>
    ) -> Timing {
        Timing {
            blocked: blocked,
            dns: dns,
            connect: connect,
            send: send,
            wait: wait,
            receive: receive,
            ssl: ssl,
            comment: comment
        }
    }
}


#[cfg(test)]
mod test {
	
	use serde_json;
    use Browser;
    use Cache;
    use CacheState::{Absent,Present,Unknown};
    use CacheEntry;
    use Content;
    use Cookie;
    use Creator;
    use Entry;
    use Header;
    use Log;
    use OptionalTiming::{TimedContent,NotApplicable};
    use Page;
    use PageTimings;
    use Param;
    use PostData;
    use QueryStringPair;
    use Request;
    use Response;
    use Timing;



    #[test]
    fn test_log() {
        let mut log = Log::new(
            Some(Browser::new("Firefox".to_string(), "3.6".to_string(), None)),
            Some("Comment".to_string())
        );
        log.add_page(Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            None
        ));
        log.add_entry(Entry::new(
            Some("page_0".to_string()),
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                NotApplicable,
                NotApplicable,
                NotApplicable,
                4,
                5,
                6,
                NotApplicable,
                None
            ),
            None,
            None,
            None
        ));
        let log_json = "{
                            \"version\": \"1.2\",
                            \"creator\": {
                                \"name\": \"Rust-HAR\",
                                \"version\": \"0.0.4\"
                            },
                            \"browser\": {
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\"
                            },
                            \"pages\": [
                                {
                                    \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                                    \"id\": \"page_0\",
                                    \"title\": \"Test Page\",
                                    \"pageTimings\": {
                                        \"onContentLoad\": null,
                                        \"onLoad\": null
                                    }
                                }
                            ],
                            \"entries\": [
                                {
                                    \"pageref\": \"page_0\",
                                    \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                                    \"request\": {
                                        \"method\": \"GET\",
                                        \"url\": \"http://www.example.com/path/?param=value\",
                                        \"httpVersion\": \"HTTP/1.1\",
                                        \"cookies\": [],
                                        \"headers\": [],
                                        \"queryString\": [],
                                        \"headersSize\": null,
                                        \"bodySize\": null
                                    },
                                    \"response\": {
                                        \"status\": 200,
                                        \"statusText\": \"OK\",
                                        \"httpVersion\": \"HTTP/1.1\",
                                        \"cookies\": [],
                                        \"headers\": [],
                                        \"content\": {
                                            \"size\": 100,
                                            \"mimeType\": \"text/html; charset=utf8\"
                                        },
                                        \"redirectUrl\": \"\",
                                        \"headersSize\": null,
                                        \"bodySize\": null
                                    },
                                    \"cache\": {
                                        \"beforeRequest\": null,
                                        \"afterRequest\": null
                                    },
                                    \"time\": 15,
                                    \"timings\": {
                                         \"blocked\": null,
                                         \"dns\": null,
                                         \"connect\": null,
                                         \"send\": 4,
                                         \"wait\": 5,
                                         \"receive\": 6,
                                         \"ssl\": null
                                    }
                                }
                            ],
                            \"comment\": \"Comment\"
                        }";
		let log_from_str: Log = serde_json::from_str(log_json).unwrap();
        assert_eq!( log_from_str, log );
    }


    #[test]
    fn test_log_no_optional() {
        let log = Log::new(None, None);
        let log_json = "{
                            \"version\": \"1.2\",
                            \"creator\": {
                                \"name\": \"Rust-HAR\",
                                \"version\": \"0.0.4\"
                            },
                            \"entries\": []
                        }";
        let log_from_str: Log = serde_json::from_str(log_json).unwrap();
        assert_eq!( log_from_str, log );
    }

    #[test]
    fn test_creator() {
        let creator = Creator::new(
            "Firebug".to_string(),
            "1.6".to_string(),
            Some("Comment".to_string())
        );
        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\",
                                \"comment\": \"Comment\"
                            }";
        let creator_from_str: Creator = serde_json::from_str(creator_json).unwrap();
        assert_eq!( creator_from_str, creator );
    }

    #[test]
    fn test_creator_no_optional() {
        let creator = Creator::new(
            "Firebug".to_string(),
            "1.6".to_string(),
            None
        );

        let creator_json = "{
                                \"name\": \"Firebug\",
                                \"version\": \"1.6\"
                            }";
        let creator_from_str: Creator = serde_json::from_str(creator_json).unwrap();
        assert_eq!( creator_from_str, creator );
    }

    #[test]
    fn test_browser() {
        let browser = Browser::new("Firefox".to_string(), "3.6".to_string(),
                                   Some("Comment".to_string()));
        let browser_json = "{
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\",
                                \"comment\": \"Comment\"
                            }";

        let browser_from_str: Browser = serde_json::from_str(browser_json).unwrap();
        assert_eq!( browser_from_str, browser );
    }

    #[test]
    fn test_browser_no_optional() {
        let browser = Browser::new("Firefox".to_string(), "3.6".to_string(), None);
        let browser_json = "{
                                \"name\": \"Firefox\",
                                \"version\": \"3.6\"
                            }";
        let browser_from_str: Browser = serde_json::from_str(browser_json).unwrap();
        assert_eq!( browser_from_str, browser );
    }

    #[test]
    fn test_page() {
        let page = Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            Some("Comment".to_string())
        );
        let page_json = "{
                             \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                             \"id\": \"page_0\",
                             \"title\": \"Test Page\",
                             \"pageTimings\": {
                                 \"onContentLoad\": null,
                                 \"onLoad\": null
                             },
                             \"comment\": \"Comment\"
                         }";
        let page_from_str: Page = serde_json::from_str(page_json).unwrap();
        assert_eq!( page_from_str, page );
    }

    #[test]
    fn test_page_no_optional() {
        let page = Page::new(
            "2009-04-16T12:07:25.123+01:00".to_string(),
            "page_0".to_string(),
            "Test Page".to_string(),
            PageTimings::new(NotApplicable, NotApplicable, None),
            None
        );
        let page_json = "{
                             \"startedDateTime\": \"2009-04-16T12:07:25.123+01:00\",
                             \"id\": \"page_0\",
                             \"title\": \"Test Page\",
                             \"pageTimings\": {
                                 \"onContentLoad\": null,
                                 \"onLoad\": null
                             }
                         }";
        let page_from_str: Page = serde_json::from_str(page_json).unwrap();
        assert_eq!( page_from_str, page );
    }

    #[test]
    fn test_page_timings() {
        let page_timings = PageTimings::new(TimedContent(1720),
                                            TimedContent(2500),
                                            Some("Comment".to_string()));
        let page_timings_json = "{
                                     \"onContentLoad\": 1720,
                                     \"onLoad\": 2500,
                                     \"comment\": \"Comment\"
                                 }";
        let page_timings_from_str: PageTimings = serde_json::from_str(page_timings_json).unwrap();
        assert_eq!(page_timings_from_str, page_timings );
    }

    #[test]
    fn test_page_timings_no_optional() {
        let page_timings = PageTimings::new(NotApplicable, NotApplicable, None);
        let page_timings_json = "{
                                     \"onContentLoad\": null,
                                     \"onLoad\": null
                                 }";
        let page_timings_from_str: PageTimings = serde_json::from_str(page_timings_json).unwrap();
        assert_eq!(page_timings_from_str, page_timings );
    }

    #[test]
    fn test_entry() {
        let entry = Entry::new(
            Some("page_0".to_string()),
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                TimedContent(1),
                TimedContent(2),
                TimedContent(3),
                4,
                5,
                6,
                TimedContent(7),
                None
            ),
            Some("10.0.0.1".to_string()),
            Some("52492".to_string()),
            Some("Comment".to_string())
        );
        let entry_json = "{
                              \"pageref\": \"page_0\",
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": []
                              },
                              \"response\": {
                                  \"status\": 200,
                                  \"statusText\": \"OK\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"content\": {
                                      \"size\": 100,
                                      \"mimeType\": \"text/html; charset=utf8\"
                                  },
                                  \"redirectUrl\": \"\"
                              },
                              \"cache\": {
                                    \"beforeRequest\": null,
                                    \"afterRequest\": null
                              },
                              \"time\": 28,
                              \"timings\": {
                                   \"blocked\": 1,
                                   \"dns\": 2,
                                   \"connect\": 3,
                                   \"send\": 4,
                                   \"wait\": 5,
                                   \"receive\": 6,
                                   \"ssl\": 7
                              },
                              \"serverIpAddress\": \"10.0.0.1\",
                              \"connection\": \"52492\",
                              \"comment\": \"Comment\"
                          }";
        let entry_from_str: Entry = serde_json::from_str(entry_json).unwrap();
        assert_eq!(entry_from_str, entry );
    }

    #[test]
    fn test_entry_no_optional() {
        let entry = Entry::new(
            None,
            "2009-04-16T12:07:23.596Z".to_string(),
            Request::new(
                "GET".to_string(),
                "http://www.example.com/path/?param=value".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                None,
                None,
                None,
                None
            ),
            Response::new(
                200,
                "OK".to_string(),
                "HTTP/1.1".to_string(),
                Vec::new(),
                Vec::new(),
                Content::new(
                    100,
                    None,
                    "text/html; charset=utf8".to_string(),
                    None,
                    None,
                    None
                ),
                "".to_string(),
                None,
                None,
                None
            ),
            Cache::new(
                Absent,
                Absent,
                None
            ),
            Timing::new(
                NotApplicable,
                NotApplicable,
                NotApplicable,
                4,
                5,
                6,
                NotApplicable,
                None
            ),
            None,
            None,
            None
        );
        let entry_json = "{
                              \"startedDateTime\": \"2009-04-16T12:07:23.596Z\",
                              \"request\": {
                                  \"method\": \"GET\",
                                  \"url\": \"http://www.example.com/path/?param=value\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"queryString\": []
                              },
                              \"response\": {
                                  \"status\": 200,
                                  \"statusText\": \"OK\",
                                  \"httpVersion\": \"HTTP/1.1\",
                                  \"cookies\": [],
                                  \"headers\": [],
                                  \"content\": {
                                      \"size\": 100,
                                      \"mimeType\": \"text/html; charset=utf8\"
                                  },
                                  \"redirectUrl\": \"\"
                              },
                              \"cache\": {
                                    \"beforeRequest\": null,
                                    \"afterRequest\": null
                              },
                              \"time\": 15,
                              \"timings\": {
                                   \"blocked\": null,
                                   \"dns\": null,
                                   \"connect\": null,
                                   \"send\": 4,
                                   \"wait\": 5,
                                   \"receive\": 6,
                                   \"ssl\": null
                              }
                          }";
        let entry_from_str: Entry = serde_json::from_str(entry_json).unwrap();
        assert_eq!(entry_from_str, entry );
        
    }

    #[test]
    fn test_request() {
        let request = Request::new(
            "GET".to_string(),
            "http://www.example.com/path/?param=value".to_string(),
            "HTTP/1.1".to_string(),
            vec![ Cookie::new(
                "TestCookie".to_string(),
                "Cookie Value".to_string(),
                None,
                None,
                None,
                None,
                None,
                None
            )],
            vec![Header::new(
                "Accept-Encoding".to_string(),
                "gzip,deflate".to_string(),
                None
            )],
            vec![QueryStringPair::new(
                "param1".to_string(),
                "value1".to_string(),
                None
            )],
            Some(PostData::new(
                "multipart/form-data".to_string(),
                Vec::new(),
                "plain posted data".to_string(),
                None
            )),
            Some(150),
            Some(0),
            Some("Comment".to_string())
        );
        let request_json = "{
                                \"method\": \"GET\",
                                \"url\": \"http://www.example.com/path/?param=value\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [{
                                    \"name\": \"TestCookie\",
                                    \"value\": \"Cookie Value\"
                                }],
                                \"headers\": [
                                    {
                                        \"name\": \"Accept-Encoding\",
                                        \"value\": \"gzip,deflate\"
                                    }
                                ],
                                \"queryString\": [
                                    {
                                          \"name\": \"param1\",
                                          \"value\": \"value1\"
                                    }
                                ],
                                \"postData\": {
                                    \"mimeType\": \"multipart/form-data\",
                                    \"params\": [],
                                    \"text\": \"plain posted data\"
                                },
                                \"headersSize\": 150,
                                \"bodySize\": 0,
                                \"comment\": \"Comment\"
                            }";
        let request_from_str: Request = serde_json::from_str(request_json).unwrap();
        assert_eq!(request_from_str, request );
    }

    #[test]
    fn test_request_no_optional() {
        let request = Request::new(
            "GET".to_string(),
            "http://www.example.com/path/?param=value".to_string(),
            "HTTP/1.1".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            None,
            None
        );
        let request_json = "{
                                \"method\": \"GET\",
                                \"url\": \"http://www.example.com/path/?param=value\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"queryString\": []
                            }";        
        let request_from_str: Request = serde_json::from_str(request_json).unwrap();
        assert_eq!(request_from_str, request );
    }

    #[test]
    fn test_response() {
        let response = Response::new(
            200, 
            "OK".to_string(), 
            "HTTP/1.1".to_string(), 
            Vec::new(), 
            Vec::new(), 
            Content::new(100, None, "text/html; charset=utf8".to_string(), None, None, None), 
            "".to_string(), 
            Some(160),
            Some(850),
            Some("".to_string())
        );
        let response_json = "{
                                \"status\": 200,
                                \"statusText\": \"OK\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"content\": {
                                    \"size\": 100,
                                    \"mimeType\": \"text/html; charset=utf8\"
                                },
                                \"redirectUrl\": \"\",
                                \"headersSize\" : 160,
                                \"bodySize\" : 850,
                                \"comment\" : \"\"
                            }";
        let response_from_str: Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response_from_str, response );
    }

    #[test]
    fn test_response_no_optional() {
        let response = Response::new(
            200, 
            "OK".to_string(), 
            "HTTP/1.1".to_string(), 
            Vec::new(), 
            Vec::new(), 
            Content::new(100, None, "text/html; charset=utf8".to_string(), None, None, None), 
            "".to_string(), 
            None,
            None,
            None
        );

        let response_json = "{
                                \"status\": 200,
                                \"statusText\": \"OK\",
                                \"httpVersion\": \"HTTP/1.1\",
                                \"cookies\": [],
                                \"headers\": [],
                                \"content\": {
                                    \"size\": 100,
                                    \"mimeType\": \"text/html; charset=utf8\"
                                },
                                \"redirectUrl\": \"\"
                            }";
        let response_from_str: Response = serde_json::from_str(response_json).unwrap();
        assert_eq!(response_from_str, response );
    }

    #[test]
    fn test_cookie() {
        let cookie = Cookie::new(
            "TestCookie".to_string(),
            "Cookie Value".to_string(), 
            Some("/".to_string()), 
            Some("www.janodvarko.cz".to_string()), 
            Some("2009-07-24T19:20:30.123+02:00".to_string()), 
            Some(false), 
            Some(false), 
            Some("".to_string())
        );
        let cookie_json = "{
                               \"name\": \"TestCookie\",
                               \"value\": \"Cookie Value\",
                               \"path\": \"/\",
                               \"domain\": \"www.janodvarko.cz\",
                               \"expires\": \"2009-07-24T19:20:30.123+02:00\",
                               \"httpOnly\": false,
                               \"secure\": false,
                               \"comment\": \"\"
                           }";
        let cookie_from_str: Cookie = serde_json::from_str(cookie_json).unwrap();
        assert_eq!(cookie_from_str, cookie );
    }

    #[test]
    fn test_cookie_no_optional() {
        let cookie = Cookie::new(
            "TestCookie".to_string(),
            "Cookie Value".to_string(),
            None,
            None,
            None,
            None,
            None,
            None
        );
        let cookie_json = "{
                               \"name\": \"TestCookie\",
                               \"value\": \"Cookie Value\"
                           }";
        let cookie_from_str: Cookie = serde_json::from_str(cookie_json).unwrap();
        assert_eq!(cookie_from_str, cookie );
    }

    #[test]
    fn test_header() {
        let header = Header::new(
            "Accept-Encoding".to_string(),
            "gzip,deflate".to_string(),
            Some("Comment".to_string())
        );
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\",
                               \"comment\": \"Comment\"
                           }";
        let header_from_str: Header = serde_json::from_str(header_json).unwrap();
        assert_eq!(header_from_str, header );
    }

    #[test]
    fn test_header_no_optional() {
        let header = Header::new(
            "Accept-Encoding".to_string(),
            "gzip,deflate".to_string(),
            None
        );
        let header_json = "{
                               \"name\": \"Accept-Encoding\",
                               \"value\": \"gzip,deflate\"
                           }";
        let header_from_str: Header = serde_json::from_str(header_json).unwrap();
        assert_eq!(header_from_str, header );
    }

    #[test]
    fn test_query_string_pair() {
        let query_string_pair = QueryStringPair::new(
            "param1".to_string(),
            "value1".to_string(),
            Some("Comment".to_string())
        );
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\",
                                          \"comment\": \"Comment\"
                                      }";
        let query_string_pair_from_str: QueryStringPair = serde_json::from_str(query_string_pair_json).unwrap();
        assert_eq!(query_string_pair_from_str, query_string_pair );
    }

    #[test]
    fn test_query_string_pair_no_optional() {
        let query_string_pair = QueryStringPair::new(
            "param1".to_string(),
            "value1".to_string(),
            None
        );
        let query_string_pair_json = "{
                                          \"name\": \"param1\",
                                          \"value\": \"value1\"
                                      }";
        let query_string_pair_from_str: QueryStringPair = serde_json::from_str(query_string_pair_json).unwrap();
        assert_eq!(query_string_pair_from_str, query_string_pair );
    }

    #[test]
    fn test_post_data() {
        let post_data = PostData::new(
            "multipart/form-data".to_string(),
            vec![Param::new( "paramName".to_string(), None, None, None, None)],
            "plain posted data".to_string(),
            Some("Comment".to_string())
        );
        let post_data_json = "{
                                  \"mimeType\": \"multipart/form-data\",
                                  \"params\": [
                                      {
                                          \"name\": \"paramName\"
                                      }
                                  ],
                                  \"text\": \"plain posted data\",
                                  \"comment\": \"Comment\"
                              }";
        let post_data_from_str: PostData = serde_json::from_str(post_data_json).unwrap();
        assert_eq!(post_data_from_str, post_data );
    }

    #[test]
    fn test_post_data_no_optional() {
        let post_data = PostData::new(
            "multipart/form-data".to_string(),
            Vec::new(),
            "plain posted data".to_string(),
            None
        );
        let post_data_json = "{
                                  \"mimeType\": \"multipart/form-data\",
                                  \"params\": [],
                                  \"text\": \"plain posted data\"
                              }";
        let post_data_from_str: PostData = serde_json::from_str(post_data_json).unwrap();
        assert_eq!(post_data_from_str, post_data );
    }

    #[test]
    fn test_param() {
        let param = Param::new(
            "paramName".to_string(),
            Some("paramValue".to_string()),
            Some("example.pdf".to_string()),
            Some("application/pdf".to_string()),
            Some("Comment".to_string())
        );
        let param_json = "{
                              \"name\": \"paramName\",
                              \"value\": \"paramValue\",
                              \"fileName\": \"example.pdf\",
                              \"contentType\": \"application/pdf\",
                              \"comment\": \"Comment\"
                          }";
        let param_from_str: Param = serde_json::from_str(param_json).unwrap();
        assert_eq!(param_from_str, param );
    }

    #[test]
    fn test_param_no_optional() {
        let param = Param::new(
            "paramName".to_string(),
            None,
            None,
            None,
            None
        );
        let param_json = "{
                              \"name\": \"paramName\"
                          }";
        let param_from_str: Param = serde_json::from_str(param_json).unwrap();
        assert_eq!(param_from_str, param );
    }

    #[test]
    fn test_content() {
        let content = Content::new(
            100, Some(0),
            "text/html; charset=utf8".to_string(),
            Some("\n".to_string()),
            Some("base64".to_string()),
            Some("Comment".to_string())
        );
        let content_json = "{
                                \"size\": 100,
                                \"compression\": 0,
                                \"mimeType\": \"text/html; charset=utf8\",
                                \"text\": \"\\n\",
                                \"encoding\": \"base64\",
                                \"comment\": \"Comment\"
                            }";
        let content_from_str: Content = serde_json::from_str(content_json).unwrap();
        assert_eq!(content_from_str, content );
    }

    #[test]
    fn test_content_no_optional() {
        let content = Content::new(
            100, None,
            "text/html; charset=utf8".to_string(),
            None,
            None,
            None
        );
        let content_json = "{
                                \"size\": 100,
                                \"mimeType\": \"text/html; charset=utf8\"
                            }";
        let content_from_str: Content = serde_json::from_str(content_json).unwrap();
        assert_eq!(content_from_str, content );
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new(
            Present(CacheEntry::new(
                None, 
                "2000-01-01T00:00:00.000Z".to_string(), 
                "123456789".to_string(),
                42,
                None
            )),
            Present(CacheEntry::new(
                None, 
                "2000-02-01T00:00:00.000Z".to_string(), 
                "987654321".to_string(), 
                24, 
                None
            )),
            Some("Comment".to_string())
        );
        let cache_json = "{
                              \"beforeRequest\": {
                                  \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                  \"eTag\": \"123456789\",
                                  \"hitCount\": 42
                              },
                              \"afterRequest\": {
                                  \"lastAccess\": \"2000-02-01T00:00:00.000Z\",
                                  \"eTag\": \"987654321\",
                                  \"hitCount\": 24
                              },
                              \"comment\": \"Comment\"
                          }";
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }

    #[test]
    fn test_cache_absent_entries() {
        let cache = Cache::new(
            Absent,
            Absent,
            None
        );
        let cache_json = "{
                              \"beforeRequest\": null,
                              \"afterRequest\": null
                          }";
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }

    #[test]
    fn test_cache_unknown_entries() {
        let cache = Cache::new(
            Unknown,
            Unknown,
            None
        );
        let cache_json = "{}";
        let cache_from_str: Cache = serde_json::from_str(cache_json).unwrap();
        assert_eq!(cache_from_str, cache );
    }


    #[test]
    fn test_cache_entry() {
        let cache_entry = CacheEntry::new(
            Some("2000-02-01T00:00:00.000Z".to_string()), 
            "2000-01-01T00:00:00.000Z".to_string(), 
            "123456789".to_string(), 
            42, 
            Some("Comment".to_string())
        );
        let cache_entry_json = "{
                                    \"expires\": \"2000-02-01T00:00:00.000Z\",
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42,
                                    \"comment\": \"Comment\"
                                }";
        let cache_entry_from_str: CacheEntry = serde_json::from_str(cache_entry_json).unwrap();
        assert_eq!(cache_entry_from_str, cache_entry );
    }

    #[test]
    fn test_cache_entry_no_optional() {
        let cache_entry = CacheEntry::new(
            None, 
            "2000-01-01T00:00:00.000Z".to_string(), 
            "123456789".to_string(), 
            42, 
            None
        );         
        let cache_entry_json = "{
                                    \"lastAccess\": \"2000-01-01T00:00:00.000Z\",
                                    \"eTag\": \"123456789\",
                                    \"hitCount\": 42
                                }";
        let cache_entry_from_str: CacheEntry = serde_json::from_str(cache_entry_json).unwrap();
        assert_eq!(cache_entry_from_str, cache_entry );
    }
    #[test]
    fn test_timing() {
        
        let timing = Timing::new(
            TimedContent(1), 
            TimedContent(2), 
            TimedContent(3), 
            4,
            5,
            6,
            TimedContent(7), 
            Some("Comment".to_string())
        );
        let timing_json = "{
                                \"blocked\": 1,
                                \"dns\": 2,
                                \"connect\": 3,
                                \"send\": 4,
                                \"wait\": 5,
                                \"receive\": 6,
                                \"ssl\": 7,
                                \"comment\":\"Comment\"
                           }";
        let timing_from_str: Timing = serde_json::from_str(timing_json).unwrap();
        assert_eq!(timing_from_str, timing );
    }

    #[test]
    fn test_timing_no_optional() {
        let timing = Timing::new(
            NotApplicable, 
            NotApplicable, 
            NotApplicable, 
            4, 
            5, 
            6, 
            NotApplicable, 
            None
        );
        let timing_json = "{
                                \"blocked\": null,
                                \"dns\": null,
                                \"connect\": null,
                                \"send\": 4,
                                \"wait\": 5,
                                \"receive\": 6,
                                \"ssl\": null
                           }";
        let timing_from_str: Timing = serde_json::from_str(timing_json).unwrap();
        assert_eq!(timing_from_str, timing );
    }
}
