#[cfg(feature = "BiRegularRectangle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRectangle")]
/// *This icon requires the feature* `BiRegularRectangle` *to be enabled*.
#[component]
pub fn Rectangle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM4 19V5h16l.001 14H4z" /></svg>
   }
}