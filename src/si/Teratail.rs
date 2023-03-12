#[cfg(feature = "SiTeratail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTeratail")]
/// *This icon requires the feature* `SiTeratail` *to be enabled*.
#[component]
pub fn Teratail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M9.81.968h4.375L24 23.032h-5.107L12.121 6.605h-.198L5.148 23.03H0Z" /></svg>
   }
}