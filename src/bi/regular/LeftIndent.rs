#[cfg(feature = "BiRegularLeftIndent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLeftIndent")]
/// *This icon requires the feature* `BiRegularLeftIndent` *to be enabled*.
#[component]
pub fn LeftIndent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 7h10v2H4zm0-4h16v2H4zm0 8h10v2H4zm0 4h10v2H4zm0 4h16v2H4zm16-3V8l-4 4z" /></svg>
   }
}