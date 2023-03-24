#[cfg(feature = "SiAdobe")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAdobe")]
/// *This icon requires the feature* `SiAdobe` *to be enabled*.
#[component]
pub fn Adobe(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M13.966 22.624l-1.69-4.281H8.122l3.892-9.144 5.662 13.425zM8.884 1.376H0v21.248zm15.116 0h-8.884L24 22.624Z" /></svg>
   }
}