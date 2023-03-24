#[cfg(feature = "SiRazorpay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiRazorpay")]
/// *This icon requires the feature* `SiRazorpay` *to be enabled*.
#[component]
pub fn Razorpay(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M22.436 0l-11.91 7.773-1.174 4.276 6.625-4.297L11.65 24h4.391l6.395-24zM14.26 10.098L3.389 17.166 1.564 24h9.008l3.688-13.902Z" /></svg>
   }
}