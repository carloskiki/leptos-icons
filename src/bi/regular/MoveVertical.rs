#[cfg(feature = "BiRegularMoveVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMoveVertical")]
/// *This icon requires the feature* `BiRegularMoveVertical` *to be enabled*.
#[component]
pub fn MoveVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m7 17 5 5 5-5h-4V7h4l-5-5-5 5h4v10z" /></svg>
   }
}