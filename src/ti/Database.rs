#[cfg(feature = "TiDatabase")]
use leptos::*;
#[cfg(feature = "TiDatabase")]
///This icon requires the feature `TiDatabase` to be enabled.
#[component]
pub fn Database(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.983 8.791c-.176-3.704-3.236-6.666-6.983-6.666s-6.807 2.962-6.983 6.666l-.017.084v6.25c0 3.86 3.141 7 7 7s7-3.14 7-7v-6.25l-.017-.084zm-6.983 8.834c-2.22 0-4.132-1.324-5-3.222v-.388c1.271 1.3 3.042 2.11 5 2.11s3.729-.811 5-2.11v.388c-.868 1.898-2.78 3.222-5 3.222zm0-13.5c2.757 0 5 2.243 5 5s-2.243 5-5 5-5-2.243-5-5 2.243-5 5-5zm0 16c-2.271 0-4.172-1.532-4.778-3.609 1.188 1.293 2.888 2.109 4.778 2.109s3.59-.816 4.778-2.109c-.606 2.077-2.507 3.609-4.778 3.609z"
        /> < title > { title } < / title > < / svg >
    }
}
