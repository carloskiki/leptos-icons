#[cfg(feature = "BiRegularDownArrowAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDownArrowAlt")]
/// *This icon requires the feature* `BiRegularDownArrowAlt` *to be enabled*.
#[component]
pub fn DownArrowAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m18.707 12.707-1.414-1.414L13 15.586V6h-2v9.586l-4.293-4.293-1.414 1.414L12 19.414z" /></svg>
   }
}