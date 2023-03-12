#[cfg(feature = "BiRegularFastForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFastForward")]
/// *This icon requires the feature* `BiRegularFastForward` *to be enabled*.
#[component]
pub fn FastForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m19 12-7-5v10zM5 7v10l7-5z" /></svg>
   }
}