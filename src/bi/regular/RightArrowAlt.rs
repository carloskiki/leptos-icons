#[cfg(feature = "BiRegularRightArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRightArrowAlt")]
/// *This icon requires the feature* `BiRegularRightArrowAlt` *to be enabled*.
#[component]
pub fn RightArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m11.293 17.293 1.414 1.414L19.414 12l-6.707-6.707-1.414 1.414L15.586 11H6v2h9.586z" /></svg>
   }
}