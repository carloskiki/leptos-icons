#[cfg(feature = "TiArrowSortedUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiArrowSortedUp")]
/// *This icon requires the feature* `TiArrowSortedUp` *to be enabled*.
#[component]
pub fn ArrowSortedUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M18.2 13.3l-6.2-6.3-6.2 6.3c-.2.2-.3.5-.3.7s.1.5.3.7c.2.2.4.3.7.3h11c.3 0 .5-.1.7-.3.2-.2.3-.5.3-.7s-.1-.5-.3-.7z" /></svg>
   }
}