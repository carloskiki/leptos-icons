#[cfg(feature = "BiRegularListUl")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularListUl")]
/// *This icon requires the feature* `BiRegularListUl` *to be enabled*.
#[component]
pub fn ListUl(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h2v2H4zm0 5h2v2H4zm0 5h2v2H4zm16-8V6H8.023v2H18.8zM8 11h12v2H8zm0 5h12v2H8z" /></svg>
   }
}