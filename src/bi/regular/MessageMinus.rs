#[cfg(feature = "BiRegularMessageMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageMinus")]
/// *This icon requires the feature* `BiRegularMessageMinus` *to be enabled*.
#[component]
pub fn MessageMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3v3.767L13.277 18H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14h-7.277L9 18.233V16H4V4h16v12z" /><path d="M8 9h8v2H8z" /></svg>
   }
}