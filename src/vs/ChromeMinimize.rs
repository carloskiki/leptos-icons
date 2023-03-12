#[cfg(feature = "VsChromeMinimize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsChromeMinimize")]
/// *This icon requires the feature* `VsChromeMinimize` *to be enabled*.
#[component]
pub fn ChromeMinimize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 8v1H3V8h11z" /></svg>
   }
}