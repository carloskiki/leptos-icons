#[cfg(feature = "BiRegularCross")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCross")]
/// *This icon requires the feature* `BiRegularCross` *to be enabled*.
#[component]
pub fn Cross(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 2h2v7h-2zm0 13h2v7h-2zm4-4h7v2h-7zM2 11h7v2H2z" /></svg>
   }
}