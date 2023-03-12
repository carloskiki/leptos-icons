#[cfg(feature = "ImCheckmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCheckmark")]
/// *This icon requires the feature* `ImCheckmark` *to be enabled*.
#[component]
pub fn Checkmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13.5 2l-7.5 7.5-3.5-3.5-2.5 2.5 6 6 10-10z" /></svg>
   }
}