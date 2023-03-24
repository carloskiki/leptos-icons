#[cfg(feature = "TiMediaPlayReverse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiMediaPlayReverse")]
/// *This icon requires the feature* `TiMediaPlayReverse` *to be enabled*.
#[component]
pub fn MediaPlayReverse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M14 19c1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2-.5 0-1 .2-1.4.6-2.6 2.5-6.6 6.4-6.6 6.4s4 3.9 6.6 6.4c.4.4.9.6 1.4.6z" /></svg>
   }
}