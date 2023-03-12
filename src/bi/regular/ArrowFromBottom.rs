#[cfg(feature = "BiRegularArrowFromBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowFromBottom")]
/// *This icon requires the feature* `BiRegularArrowFromBottom` *to be enabled*.
#[component]
pub fn ArrowFromBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 18h12v2H6zm6-14.414-6.707 6.707 1.414 1.414L11 7.414V16h2V7.414l4.293 4.293 1.414-1.414z" /></svg>
   }
}