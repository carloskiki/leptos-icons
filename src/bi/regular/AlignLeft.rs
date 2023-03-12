#[cfg(feature = "BiRegularAlignLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlignLeft")]
/// *This icon requires the feature* `BiRegularAlignLeft` *to be enabled*.
#[component]
pub fn AlignLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 19h16v2H4zm0-4h11v2H4zm0-4h16v2H4zm0-8h16v2H4zm0 4h11v2H4z" /></svg>
   }
}