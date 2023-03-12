#[cfg(feature = "ImHangouts")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImHangouts")]
/// *This icon requires the feature* `ImHangouts` *to be enabled*.
#[component]
pub fn Hangouts(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M7.997 0c-3.816 0-6.909 3.094-6.909 6.909 0 3.616 3.294 6.547 6.909 6.547v2.544c4.197-2.128 6.916-5.556 6.916-9.091 0-3.816-3.1-6.909-6.916-6.909zM7 8c0 0.828-0.447 1.5-1 1.5v-1.5h-2v-3h3v3zM12 8c0 0.828-0.447 1.5-1 1.5v-1.5h-2v-3h3v3z" /></svg>
   }
}