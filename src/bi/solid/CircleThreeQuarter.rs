#[cfg(feature = "BiSolidCircleThreeQuarter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCircleThreeQuarter")]
/// *This icon requires the feature* `BiSolidCircleThreeQuarter` *to be enabled*.
#[component]
pub fn CircleThreeQuarter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2h-1v9H2v1a10 10 0 0 0 17.07 7.07A10 10 0 0 0 12 2z" /></svg>
   }
}