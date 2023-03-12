#[cfg(feature = "SiUnsplash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiUnsplash")]
/// *This icon requires the feature* `SiUnsplash` *to be enabled*.
#[component]
pub fn Unsplash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M7.5 6.75V0h9v6.75h-9zm9 3.75H24V24H0V10.5h7.5v6.75h9V10.5z" /></svg>
   }
}