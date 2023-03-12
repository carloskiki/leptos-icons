#[cfg(feature = "ImPause2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPause2")]
/// *This icon requires the feature* `ImPause2` *to be enabled*.
#[component]
pub fn Pause2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M2 2h5v12h-5zM9 2h5v12h-5z" /></svg>
   }
}