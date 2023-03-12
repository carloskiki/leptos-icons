#[cfg(feature = "ImLocation2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLocation2")]
/// *This icon requires the feature* `ImLocation2` *to be enabled*.
#[component]
pub fn Location2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c-2.761 0-5 2.239-5 5 0 5 5 11 5 11s5-6 5-11c0-2.761-2.239-5-5-5zM8 8.063c-1.691 0-3.063-1.371-3.063-3.063s1.371-3.063 3.063-3.063 3.063 1.371 3.063 3.063-1.371 3.063-3.063 3.063zM6.063 5c0-1.070 0.867-1.938 1.938-1.938s1.938 0.867 1.938 1.938c0 1.070-0.867 1.938-1.938 1.938s-1.938-0.867-1.938-1.938z" /></svg>
   }
}