#[cfg(feature = "ImFlickr2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFlickr2")]
/// *This icon requires the feature* `ImFlickr2` *to be enabled*.
#[component]
pub fn Flickr2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M12.5 6.5c-1.103 0-2 0.897-2 2s0.897 2 2 2c1.103 0 2-0.897 2-2s-0.897-2-2-2zM12.5 5v0c1.933 0 3.5 1.567 3.5 3.5s-1.567 3.5-3.5 3.5-3.5-1.567-3.5-3.5c0-1.933 1.567-3.5 3.5-3.5zM0 8.5c0-1.933 1.567-3.5 3.5-3.5s3.5 1.567 3.5 3.5c0 1.933-1.567 3.5-3.5 3.5s-3.5-1.567-3.5-3.5z" /></svg>
   }
}