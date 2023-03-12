#[cfg(feature = "ImOpt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImOpt")]
/// *This icon requires the feature* `ImOpt` *to be enabled*.
#[component]
pub fn Opt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14.5 13h-4c-0.198 0-0.377-0.116-0.457-0.297l-3.868-8.703h-4.675c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h5c0.198 0 0.377 0.116 0.457 0.297l3.868 8.703h3.675c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z" /><path fill="#000000" d="M14.5 4h-5c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h5c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z" /></svg>
   }
}