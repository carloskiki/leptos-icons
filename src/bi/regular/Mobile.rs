#[cfg(feature = "BiRegularMobile")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMobile")]
/// *This icon requires the feature* `BiRegularMobile` *to be enabled*.
#[component]
pub fn Mobile(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 2H7c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h10c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM7 16.999V5h10l.002 11.999H7z" /></svg>
   }
}