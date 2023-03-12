#[cfg(feature = "TbRecordMail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRecordMail")]
/// *This icon requires the feature* `TbRecordMail` *to be enabled*.
#[component]
pub fn RecordMail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-record-mail" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M17 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><path d="M7 15l10 0" /></svg>
   }
}