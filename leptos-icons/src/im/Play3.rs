#[cfg(feature = "ImPlay3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPlay3")]
/// *This icon requires the feature* `ImPlay3` *to be enabled*.
#[component]
pub fn Play3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 2l10 6-10 6z" /></svg>
   }
}