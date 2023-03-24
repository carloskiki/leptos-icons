#[cfg(feature = "OcLgDotFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgDotFill")]
/// *This icon requires the feature* `OcLgDotFill` *to be enabled*.
#[component]
pub fn DotFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 18a6 6 0 1 0 0-12 6 6 0 0 0 0 12Z" /></svg>
   }
}