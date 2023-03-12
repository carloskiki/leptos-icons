#[cfg(feature = "BiRegularMoveHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMoveHorizontal")]
/// *This icon requires the feature* `BiRegularMoveHorizontal` *to be enabled*.
#[component]
pub fn MoveHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17 11H7V7l-5 5 5 5v-4h10v4l5-5-5-5z" /></svg>
   }
}