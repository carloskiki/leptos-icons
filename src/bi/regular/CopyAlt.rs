#[cfg(feature = "BiRegularCopyAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCopyAlt")]
/// *This icon requires the feature* `BiRegularCopyAlt` *to be enabled*.
#[component]
pub fn CopyAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H10c-1.103 0-2 .897-2 2v4H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h10c1.103 0 2-.897 2-2v-4h4c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM4 20V10h10l.002 10H4zm16-6h-4v-4c0-1.103-.897-2-2-2h-4V4h10v10z" /><path d="M6 12h6v2H6zm0 4h6v2H6z" /></svg>
   }
}