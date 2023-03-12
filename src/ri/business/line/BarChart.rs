#[cfg(feature = "RiBusinessLineBarChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessLineBarChart")]
/// *This icon requires the feature* `RiBusinessLineBarChart` *to be enabled*.
#[component]
pub fn BarChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 12h2v9H3v-9zm16-4h2v13h-2V8zm-8-6h2v19h-2V2z" /></g></svg>
   }
}