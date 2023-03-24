#[cfg(feature = "SiOsano")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiOsano")]
/// *This icon requires the feature* `SiOsano` *to be enabled*.
#[component]
pub fn Osano(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 6.091A5.909 5.909 0 1017.909 12 5.91 5.91 0 0012 6.091M12 0A12 12 0 110 12 12 12 0 0112 0z" /></svg>
   }
}