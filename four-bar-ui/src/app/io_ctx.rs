#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsValue};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn save_file(s: &str, file_name: &str);
    fn load_file(format: &str, done: JsValue);
    fn get_host() -> String;
    fn get_username() -> String;
    fn login(account: &str, body: &str, done: JsValue);
    fn logout(done: JsValue);
}

#[derive(Clone)]
pub(crate) struct IoCtx {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) agent: ureq::Agent,
}

impl Default for IoCtx {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            agent: ureq::Agent::new(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl IoCtx {
    pub(crate) fn open<C>(&self, _fmt: &str, ext: &[&str], done: C)
    where
        C: FnOnce(String) + 'static,
    {
        let format = ext
            .iter()
            .map(|s| format!(".{}", s))
            .collect::<Vec<_>>()
            .join(",");
        load_file(&format, Closure::once_into_js(done));
    }

    pub(crate) fn save(&self, s: &str, file_name: &str) {
        save_file(s, file_name);
    }

    pub(crate) fn alert(s: &str) {
        alert(s);
    }

    pub(crate) fn get_username(&self, _url: &str) -> Option<String> {
        Some(get_username())
    }

    pub(crate) fn login<C>(&self, _url: &str, account: &str, body: &str, done: C)
    where
        C: FnOnce(bool) + 'static,
    {
        login(account, body, Closure::once_into_js(done));
    }

    pub(crate) fn logout<C>(&self, _url: &str, done: C)
    where
        C: FnOnce(bool) + 'static,
    {
        logout(Closure::once_into_js(done));
    }

    pub(crate) fn get_host() -> String {
        get_host()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl IoCtx {
    pub(crate) fn open<C>(&self, fmt: &str, ext: &[&str], done: C)
    where
        C: FnOnce(String) + 'static,
    {
        let s = match rfd::FileDialog::new().add_filter(fmt, ext).pick_file() {
            Some(path) => std::fs::read_to_string(path).unwrap_or_default(),
            None => String::new(),
        };
        done(s);
    }

    pub(crate) fn save(&self, s: &str, name: &str, fmt: &str, ext: &[&str]) {
        if let Some(file_name) = rfd::FileDialog::new()
            .set_file_name(name)
            .add_filter(fmt, ext)
            .save_file()
        {
            std::fs::write(file_name, s).unwrap_or_default();
        }
    }

    pub(crate) fn alert(s: &str) {
        rfd::MessageDialog::new()
            .set_title("Message")
            .set_description(s)
            .show();
    }

    pub(crate) fn get_username(&self, url: &str) -> Option<String> {
        if self.agent.get(url).call().is_err() {
            None
        } else if let Ok(uri) = url.parse::<actix_web::http::Uri>() {
            match self
                .agent
                .cookie_store()
                .get(uri.host().unwrap_or_default(), "/", "username")
            {
                Some(name) => Some(name.value().to_string()),
                None => Some(String::new()),
            }
        } else {
            None
        }
    }

    pub(crate) fn login<C>(&self, url: &str, account: &str, body: &str, done: C)
    where
        C: FnOnce(bool) + 'static,
    {
        let b = self
            .agent
            .post(&[url.trim_end_matches('/'), "login", account].join("/"))
            .set("content-type", "application/json")
            .send_bytes(body.as_bytes())
            .is_ok();
        done(b);
    }

    pub(crate) fn logout<C>(&self, url: &str, done: C)
    where
        C: FnOnce(bool) + 'static,
    {
        let b = self
            .agent
            .post(&[url.trim_end_matches('/'), "logout"].join("/"))
            .call()
            .is_ok();
        done(b);
    }
}
