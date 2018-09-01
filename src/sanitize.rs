use clipboard::{ClipboardContext, ClipboardProvider};
use std::thread;
use std::time::Duration;
use url::Url;

/// Polls the system's clipboard and tries to sanitize URLs from tracking information.
pub fn sanitize_loop(interval: Duration) {
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to initialize clipboard.");

    let mut current_clipboard_cnts = String::new();

    loop {
        let clipboard_contents = ctx
            .get_contents()
            .expect("Failed to get clipboard contents.");

        if clipboard_contents == current_clipboard_cnts {
            thread::sleep(interval);
            continue;
        }

        let mut url = match Url::parse(&clipboard_contents) {
            Ok(url) => url,
            Err(_) => {
                current_clipboard_cnts = clipboard_contents;
                continue;
            }
        };

        sanitize_url(&mut url);

        ctx.set_contents(url.to_string())
            .expect("Failed to set clipboard contents.");
        current_clipboard_cnts = url.into_string();
    }
}

/// Sanitizes the given URL's query parameters removing all tracking information.
fn sanitize_url(input: &mut Url) {
    let new_query = input.query_pairs()
        .into_owned()
        .filter(|(k, _)| !k.starts_with("utm_")) // Google Analytics
        .filter(|(k, _)| !k.starts_with("si")) // Spotify
        .collect::<Vec<_>>();

    input.query_pairs_mut().clear().extend_pairs(new_query);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_url_smoke() {
        let mut url = "https://example.com/?utm_source=bla&utm_medium=hahaha"
            .parse()
            .unwrap();
        let clean = "https://example.com/?".parse().unwrap();

        sanitize_url(&mut url);
        assert_eq!(url, clean);
    }

    #[test]
    fn sanitize_url_dont_remove_all() {
        let mut url = "https://example.com/?utm_source=bla&utm_medium=hahaha&test=hahah"
            .parse()
            .unwrap();
        let clean = "https://example.com/?test=hahah".parse().unwrap();

        sanitize_url(&mut url);
        assert_eq!(url, clean);
    }
}
