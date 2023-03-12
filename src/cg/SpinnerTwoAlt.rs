#[cfg(feature = "CgSpinnerTwoAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSpinnerTwoAlt")]
/// *This icon requires the feature* `CgSpinnerTwoAlt` *to be enabled*.
#[component]
pub fn SpinnerTwoAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 22C17.5228 22 22 17.5228 22 12H19C19 15.866 15.866 19 12 19V22Z" fill="currentColor" /><path d="M2 12C2 6.47715 6.47715 2 12 2V5C8.13401 5 5 8.13401 5 12H2Z" fill="currentColor" /></svg>
   }
}