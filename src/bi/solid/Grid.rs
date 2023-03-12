#[cfg(feature = "BiSolidGrid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidGrid")]
/// *This icon requires the feature* `BiSolidGrid` *to be enabled*.
#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 4h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4zM4 10h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4zM4 16h4v4H4zm6 0h4v4h-4zm6 0h4v4h-4z" /></svg>
   }
}