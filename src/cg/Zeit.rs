#[cfg(feature = "CgZeit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgZeit")]
/// *This icon requires the feature* `CgZeit` *to be enabled*.
#[component]
pub fn Zeit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M17.9923 17.0225L11.9806 6.97729L6.00775 17.0225H17.9923Z" fill="currentColor" /></svg>
   }
}