#[cfg(feature = "BiSolidCctv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCctv")]
/// *This icon requires the feature* `BiSolidCctv` *to be enabled*.
#[component]
pub fn Cctv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.618 7.462 6.403 2.085a1.007 1.007 0 0 0-.77-.016 1.002 1.002 0 0 0-.552.537l-3 7a1 1 0 0 0 .525 1.313L9.563 13.9 8.323 17H4v-3H2v8h2v-3h4.323c.823 0 1.552-.494 1.856-1.258l1.222-3.054 3.419 1.465a1 1 0 0 0 1.311-.518l3-6.857a1 1 0 0 0-.513-1.316zm1.312 8.91-1.858-.742 1.998-5 1.858.741z" /></svg>
   }
}