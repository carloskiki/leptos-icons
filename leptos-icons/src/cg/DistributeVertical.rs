#[cfg(feature = "CgDistributeVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDistributeVertical")]
/// *This icon requires the feature* `CgDistributeVertical` *to be enabled*.
#[component]
pub fn DistributeVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M9 11H15V13H9V11Z" stroke="currentColor" stroke-opacity="0.5" stroke-width="2" /><path d="M19 7H5V5H19V7Z" fill="currentColor" /><path d="M19 19H5V17H19V19Z" fill="currentColor" /></svg>
   }
}