#[cfg(feature = "TbGenderDemigirl")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbGenderDemigirl")]
/// *This icon requires the feature* `TbGenderDemigirl` *to be enabled*.
#[component]
pub fn GenderDemigirl(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-gender-demigirl" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 9m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M12 14v7" /><path d="M9 18h3" /></svg>
   }
}