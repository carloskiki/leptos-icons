#[cfg(feature = "SiVbulletin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVbulletin")]
/// *This icon requires the feature* `SiVbulletin` *to be enabled*.
#[component]
pub fn Vbulletin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.09 10.316V24h21.82V0h-2.417l-5.461 19.613h-6.09l-3.134-9.246zm9.283-4.444l1.363 6.308L13.955 0H1.089v5.872Z" /></svg>
   }
}