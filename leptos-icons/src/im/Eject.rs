#[cfg(feature = "ImEject")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEject")]
/// *This icon requires the feature* `ImEject` *to be enabled*.
#[component]
pub fn Eject(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 12h16v2h-16zM8 2l8 8h-16z" /></svg>
   }
}