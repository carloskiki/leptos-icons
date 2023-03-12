#[cfg(feature = "BiRegularFirstPage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFirstPage")]
/// *This icon requires the feature* `BiRegularFirstPage` *to be enabled*.
#[component]
pub fn FirstPage(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m16.293 17.707 1.414-1.414L13.414 12l4.293-4.293-1.414-1.414L10.586 12zM7 6h2v12H7z" /></svg>
   }
}