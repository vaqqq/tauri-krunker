#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, WebviewWindowBuilder, WebviewUrl};

fn main() {

    let block_script = r#"
    const blocked = [
      'fran-cdn.frvr.com/pubads_',
      'fran-cdn.frvr.com/gpt_',
      'fran-cdn.frvr.com/prebid',
      'apis.google.com/js/platform.js',
      'cookiepro.com',
      'platform.twitter.com',
      'c.amazon-adsystem.com',
      'doubleclick.net',
      'googlesyndication.com',
      'cpmstar.com',
      'adsafeprotected.com',
      'amazon-adsystem.com',
      'iionads.com',
      'pubmatic.com',
      'adnxs.com',
      'criteo.com',
      'rubiconproject.com',
      'e-planning.net'
    ];

    const originalFetch = window.fetch;
    window.fetch = function (...args) {
      let url = "";
      if (typeof args[0] === "string") {
        url = args[0];
      } else if (args[0] instanceof Request) {
        url = args[0].url;
      }

      if (blocked.some(b => url.includes(b))) {
        return Promise.reject("Blocked");
      }
      return originalFetch.apply(this, args);
    };

    const originalOpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function (method, url) {
      if (blocked.some(b => url.includes(b))) {
        return;
      }
      return originalOpen.apply(this, arguments);
    };
    "#;

    Builder::default()
        .setup(move |app| {
            let _window = WebviewWindowBuilder::new(
                app,
                "krunker",
                WebviewUrl::External("https://krunker.io".parse().unwrap()),
            )
            .title("Krunker Client")
            .inner_size(1280.0, 720.0)
            .resizable(true)
            .initialization_script(block_script)
            //.additional_browser_args("--disable-frame-rate-limit --disable-gpu-vsync --max-gum-fps=9999") // causing lags while shooting
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running Tauri app");
}
