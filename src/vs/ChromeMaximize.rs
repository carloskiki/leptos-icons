#[cfg(feature = "VsChromeMaximize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChromeMaximize")]
/// *This icon requires the feature* `VsChromeMaximize` *to be enabled*.
#[component]
pub fn ChromeMaximize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3 3v10h10V3H3zm9 9H4V4h8v8z" /></svg>
   }
}