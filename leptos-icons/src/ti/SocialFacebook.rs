#[cfg(feature = "TiSocialFacebook")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiSocialFacebook")]
/// *This icon requires the feature* `TiSocialFacebook` *to be enabled*.
#[component]
pub fn SocialFacebook(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M13 10h3v3h-3v7h-3v-7h-3v-3h3v-1.255c0-1.189.374-2.691 1.118-3.512.744-.823 1.673-1.233 2.786-1.233h2.096v3h-2.1c-.498 0-.9.402-.9.899v2.101z" /></svg>
   }
}