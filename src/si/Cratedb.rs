#[cfg(feature = "SiCratedb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCratedb")]
/// *This icon requires the feature* `SiCratedb` *to be enabled*.
#[component]
pub fn Cratedb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M18 9V3h-6v6H0v6h6v6h6v-6h12V9h-6z" /></svg>
   }
}