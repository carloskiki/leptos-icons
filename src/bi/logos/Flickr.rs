#[cfg(feature = "BiLogosFlickr")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosFlickr")]
/// *This icon requires the feature* `BiLogosFlickr` *to be enabled*.
#[component]
pub fn Flickr(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.157 12a4.573 4.573 0 1 1-9.147 0 4.573 4.573 0 0 1 9.147 0zm10.833 0a4.573 4.573 0 1 1-9.147 0 4.573 4.573 0 0 1 9.147 0z" /></svg>
   }
}