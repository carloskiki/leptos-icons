#[cfg(feature = "ImMap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMap")]
/// *This icon requires the feature* `ImMap` *to be enabled*.
#[component]
pub fn Map(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 3l5-2v12l-5 2z" /><path fill="#000000" d="M6 0.5l5 3v11.5l-5-2.5z" /><path fill="#000000" d="M12 3.5l4-3v12l-4 3z" /></svg>
   }
}