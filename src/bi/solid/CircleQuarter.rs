#[cfg(feature = "BiSolidCircleQuarter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCircleQuarter")]
/// *This icon requires the feature* `BiSolidCircleQuarter` *to be enabled*.
#[component]
pub fn CircleQuarter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2h-1v11h11v-1A10 10 0 0 0 12 2z" /></svg>
   }
}