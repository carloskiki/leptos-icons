#[cfg(feature = "FiCreditCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCreditCard")]
/// *This icon requires the feature* `FiCreditCard` *to be enabled*.
#[component]
pub fn CreditCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="1" y="4" width="22" height="16" rx="2" ry="2" /><line x1="1" y1="10" x2="23" y2="10" /></svg>
   }
}