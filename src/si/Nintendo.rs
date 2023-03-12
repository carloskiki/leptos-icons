#[cfg(feature = "SiNintendo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiNintendo")]
/// *This icon requires the feature* `SiNintendo` *to be enabled*.
#[component]
pub fn Nintendo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 .6h7.1l9.85 15.9V.6H24v22.8h-7.04L7.06 7.5v15.9H0V.6" /></svg>
   }
}