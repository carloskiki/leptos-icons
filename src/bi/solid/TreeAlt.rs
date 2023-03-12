#[cfg(feature = "BiSolidTreeAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTreeAlt")]
/// *This icon requires the feature* `BiSolidTreeAlt` *to be enabled*.
#[component]
pub fn TreeAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m18 10-6-8-6 8h3l-5 8h7v4h2v-4h7l-5-8h3z" /></svg>
   }
}