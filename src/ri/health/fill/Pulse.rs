#[cfg(feature = "RiHealthFillPulse")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiHealthFillPulse")]
/// *This icon requires the feature* `RiHealthFillPulse` *to be enabled*.
#[component]
pub fn Pulse(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M9 7.539L15 21.539 18.659 13 23 13 23 11 17.341 11 15 16.461 9 2.461 5.341 11 1 11 1 13 6.659 13z" /></g></svg>
   }
}