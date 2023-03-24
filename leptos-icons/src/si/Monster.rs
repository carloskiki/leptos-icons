#[cfg(feature = "SiMonster")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMonster")]
/// *This icon requires the feature* `SiMonster` *to be enabled*.
#[component]
pub fn Monster(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0V24H5.42V12.39L12 18.19L18.58 12.39V24H24V0L12 11.23L0 0Z" /></svg>
   }
}