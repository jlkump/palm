use forgejo_api::{Auth, Forgejo};
use url::Url;

struct ForgejoInstance {
    api: Forgejo,
}

impl ForgejoInstance {
    pub fn new<'a>(forgejo_url: &'a str) -> Self {
        Self {
            api: Forgejo::new(Auth::None, Url::parse(forgejo_url).unwrap()).unwrap(),
        }
    }
}
