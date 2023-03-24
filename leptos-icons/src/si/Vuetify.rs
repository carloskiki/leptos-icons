#[cfg(feature = "SiVuetify")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiVuetify")]
/// *This icon requires the feature* `SiVuetify` *to be enabled*.
#[component]
pub fn Vuetify(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M7.094 0L12 11.596 16.906 0H7.094zM1.5 3.5L12 24 22.5 3.5H17L12 15 7 3.5z" /></svg>
   }
}