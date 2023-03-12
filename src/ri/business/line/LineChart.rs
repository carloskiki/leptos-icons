#[cfg(feature = "RiBusinessLineLineChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessLineLineChart")]
/// *This icon requires the feature* `RiBusinessLineLineChart` *to be enabled*.
#[component]
pub fn LineChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M5 3v16h16v2H3V3h2zm15.293 3.293l1.414 1.414L16 13.414l-3-2.999-4.293 4.292-1.414-1.414L13 7.586l3 2.999 4.293-4.292z" /></g></svg>
   }
}