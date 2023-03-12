#[cfg(feature = "BiLogosVuejs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosVuejs")]
/// *This icon requires the feature* `BiLogosVuejs` *to be enabled*.
#[component]
pub fn Vuejs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 12.765 5.592-9.437h-3.276L12 7.33v.002L9.688 3.328h-3.28z" /><path d="M18.461 3.332 12 14.235 5.539 3.332H1.992L12 20.672l10.008-17.34z" /></svg>
   }
}