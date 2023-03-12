#[cfg(feature = "FaRegularCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaRegularCircle")]
/// *This icon requires the feature* `FaRegularCircle` *to be enabled*.
#[component]
pub fn Circle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256zM256 48C141.1 48 48 141.1 48 256C48 370.9 141.1 464 256 464C370.9 464 464 370.9 464 256C464 141.1 370.9 48 256 48z" /></svg>
   }
}