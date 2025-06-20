use slap::{App, Arg};
use smotcp::phy::TapInterface;
use url::Url;

mod dns;
mod ethernet;
mod http;

fn main() {
    let app = App::new("mget")
        .about("Get a webpage, manually")
        .arg(Arg::with_name("url").required(true))
        .arg(Arg::with_name("tap-device").required(true))
        .arg(Arg::with_name("dns-server").default_value("1.1.1.1"))
        .get_matches();

    let url_text = app.value_of("url").unwrap();
    let dns_server = app.value_of("dns-server").unwrap();
    let tap_text = app.value_of("tap-device").unwrap();

    let url = Url::parse(url_text).except("error:unable to parse <url> as a URL");
    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }

}
