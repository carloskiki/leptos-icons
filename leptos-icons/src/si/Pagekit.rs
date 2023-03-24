#[cfg(feature = "SiPagekit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPagekit")]
/// *This icon requires the feature* `SiPagekit` *to be enabled*.
#[component]
pub fn Pagekit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M2.401 0v24h9.6v-3.527H5.929V3.526h12.146v13.421h-6.073v3.525H21.6V0H2.401z" /></svg>
   }
}