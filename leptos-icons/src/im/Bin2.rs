#[cfg(feature = "ImBin2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImBin2")]
/// *This icon requires the feature* `ImBin2` *to be enabled*.
#[component]
pub fn Bin2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M3 16h10l1-11h-12zM10 2v-2h-4v2h-5v3l1-1h12l1 1v-3h-5zM9 2h-2v-1h2v1z" /></svg>
   }
}