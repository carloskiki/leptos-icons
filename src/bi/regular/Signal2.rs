#[cfg(feature = "BiRegularSignal2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSignal2")]
/// *This icon requires the feature* `BiRegularSignal2` *to be enabled*.
#[component]
pub fn Signal2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 16h2v5H3zm4-3h2v8H7z" /></svg>
   }
}