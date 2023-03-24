#[cfg(feature = "ImBackward2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBackward2")]
/// *This icon requires the feature* `ImBackward2` *to be enabled*.
#[component]
pub fn Backward2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 2.5v5l5-5v11l-5-5v5l-5.5-5.5z" /></svg>
   }
}