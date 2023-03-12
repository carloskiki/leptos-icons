#[cfg(feature = "BiRegularHorizontalCenter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularHorizontalCenter")]
/// *This icon requires the feature* `BiRegularHorizontalCenter` *to be enabled*.
#[component]
pub fn HorizontalCenter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m5.005 15.995 4-4-4-4v3h-3v2h3zm14-5v-3l-4 4 4 4v-3h3v-2h-2.072zm-8 7h2v3h-2zm0-5h2v3h-2zm0-5h2v3h-2zm0-5h2v3h-2z" /></svg>
   }
}