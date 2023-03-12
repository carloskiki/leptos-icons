#[cfg(feature = "BiSolidRectangle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidRectangle")]
/// *This icon requires the feature* `BiSolidRectangle` *to be enabled*.
#[component]
pub fn Rectangle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 20h18a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v14a1 1 0 0 0 1 1z" /></svg>
   }
}