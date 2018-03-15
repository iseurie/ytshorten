extern crate url;
use ::std::io::{self, Read, Write};
use ::std::borrow::Cow;
use ::url::Url;
use ::std::env;


fn main() {
    let argv = env::args().collect::<Vec<_>>();
    let flag = |strs: &[&str]| {
        strs.iter()
            .map(|s| (*s).to_owned())
            .map(|s| argv.contains(&s))
            .fold(false, |acc, b| acc || b)
    };
    let stdout = io::stdout();
    let mut writelck = stdout.lock();
    let ids_only = flag(&["-i", "--ids-only"]);
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    for s in buf.split_whitespace() {
        let uri = Url::parse(s).expect("URI parse");
        let mut id_iter = uri.query_pairs()
            .filter(|&(ref key, _)| key == &Cow::Borrowed("v"))
            .map(|(_, val)| val);
        if let Some(Cow::Borrowed(id)) = id_iter.next() {
            if ids_only {
                writelck.write(b"https://youtu.be/");
            }
            writelck.write(id.as_bytes());
            writelck.write(b"\n");
        } else {
            eprintln!("No `v` query param parsed: {}", uri.as_str());
        }
    }
}
