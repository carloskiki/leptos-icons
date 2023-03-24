#[cfg(feature = "SiKibana")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKibana")]
/// *This icon requires the feature* `SiKibana` *to be enabled*.
#[component]
pub fn Kibana(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M2.625 0v21.591L21.375 0zm10.864 12.47L3.477 24h17.522a18.755 18.755 0 0 0-7.51-11.53z" /></svg>
   }
}