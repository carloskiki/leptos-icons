#[cfg(feature = "BiRegularHeading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularHeading")]
/// *This icon requires the feature* `BiRegularHeading` *to be enabled*.
#[component]
pub fn Heading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 20V4h-3v6H9V4H6v16h3v-7h6v7z" /></svg>
   }
}