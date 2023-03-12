#[cfg(feature = "BsTabletFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsTabletFill")]
/// *This icon requires the feature* `BsTabletFill` *to be enabled*.
#[component]
pub fn TabletFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-tablet-fill" viewBox="0 0 16 16"><path d="M2 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2zm7 11a1 1 0 1 0-2 0 1 1 0 0 0 2 0z" /></svg>
   }
}