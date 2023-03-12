#[cfg(feature = "BiRegularDetail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDetail")]
/// *This icon requires the feature* `BiRegularDetail` *to be enabled*.
#[component]
pub fn Detail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM4 19V5h16l.002 14H4z" /><path d="M6 7h12v2H6zm0 4h12v2H6zm0 4h6v2H6z" /></svg>
   }
}