#[cfg(feature = "BiRegularExpandHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExpandHorizontal")]
/// *This icon requires the feature* `BiRegularExpandHorizontal` *to be enabled*.
#[component]
pub fn ExpandHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8.29 5.64 1.93 12l6.36 6.36 1.42-1.41L4.76 12l4.95-4.95-1.42-1.41zm6 1.41L19.24 12l-4.95 4.95 1.42 1.41L22.07 12l-6.36-6.36-1.42 1.41z" /></svg>
   }
}