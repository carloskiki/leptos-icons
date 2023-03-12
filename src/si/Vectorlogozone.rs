#[cfg(feature = "SiVectorlogozone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVectorlogozone")]
/// *This icon requires the feature* `SiVectorlogozone` *to be enabled*.
#[component]
pub fn Vectorlogozone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.458 0l-5.311 2.024 1.989.534-4.847 16.085-4.867-16.25H1.48L8.974 24h4.645l7.043-20.226 1.858.499Z" /></svg>
   }
}