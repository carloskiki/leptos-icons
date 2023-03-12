#[cfg(feature = "SiHouzz")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiHouzz")]
/// *This icon requires the feature* `SiHouzz` *to be enabled*.
#[component]
pub fn Houzz(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.27 0V24H9.32V16.44H14.68V24H22.73V10.37L6.61 5.75V0H1.27Z" /></svg>
   }
}