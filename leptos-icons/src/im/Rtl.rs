#[cfg(feature = "ImRtl")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImRtl")]
/// *This icon requires the feature* `ImRtl` *to be enabled*.
#[component]
pub fn Rtl(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4 0c-2.209 0-4 1.791-4 4s1.791 4 4 4v8h2v-14h2v14h2v-14h2v-2h-8zM16 3l-4 4 4 4z" /></svg>
   }
}