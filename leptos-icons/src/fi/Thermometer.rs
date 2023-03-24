#[cfg(feature = "FiThermometer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiThermometer")]
/// *This icon requires the feature* `FiThermometer` *to be enabled*.
#[component]
pub fn Thermometer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z" /></svg>
   }
}