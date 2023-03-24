#[cfg(feature = "SiSparkar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSparkar")]
/// *This icon requires the feature* `SiSparkar` *to be enabled*.
#[component]
pub fn Sparkar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M3.199 20.001L20.801 12v8.001L11.999 24l-8.8-3.999zm8.8 3.999zm-.001-24L3.199 3.999V12l17.602-8.001L11.998 0zM3.803 12.275l7.592 3.453 8.803-4.002-7.594-3.45-8.801 3.999z" /></svg>
   }
}