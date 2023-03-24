#[cfg(feature = "ImCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCrop")]
/// *This icon requires the feature* `ImCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M13 4l3-3-1-1-3 3h-7v-3h-2v3h-3v2h3v8h8v3h2v-3h3v-2h-3v-7zM5 5h5l-5 5v-5zM6 11l5-5v5h-5z" /></svg>
   }
}