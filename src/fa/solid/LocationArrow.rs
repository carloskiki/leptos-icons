#[cfg(feature = "FaSolidLocationArrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidLocationArrow")]
/// *This icon requires the feature* `FaSolidLocationArrow` *to be enabled*.
#[component]
pub fn LocationArrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M413.6 92.1c4.9-11.9 2.1-25.6-7-34.7s-22.8-11.9-34.7-7l-352 144C5.7 200.2-2.3 215.2 .6 230.2s16.1 25.8 31.4 25.8H208V432c0 15.3 10.8 28.4 25.8 31.4s30-5.1 35.8-19.3l144-352z" /></svg>
   }
}