#[cfg(feature = "AiFilledContainer")]
use leptos::*;
#[cfg(feature = "AiFilledContainer")]
///This icon requires the feature `AiFilledContainer` to be enabled.
#[component]
pub fn Container(
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
        stroke_witdh = "0" style = style class = "icon" viewBox = "0 0 1024 1024" width =
        { size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg"
        d =
        "M832 64H192c-17.7 0-32 14.3-32 32v529c0-.6.4-1 1-1h219.3l5.2 24.7C397.6 708.5 450.8 752 512 752s114.4-43.5 126.4-103.3l5.2-24.7H863c.6 0 1 .4 1 1V96c0-17.7-14.3-32-32-32zM712 493c0 4.4-3.6 8-8 8H320c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h384c4.4 0 8 3.6 8 8v48zm0-160c0 4.4-3.6 8-8 8H320c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h384c4.4 0 8 3.6 8 8v48zm151 354H694.1c-11.6 32.8-32 62.3-59.1 84.7-34.5 28.6-78.2 44.3-123 44.3s-88.5-15.8-123-44.3a194.02 194.02 0 0 1-59.1-84.7H161c-.6 0-1-.4-1-1v242c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V686c0 .6-.4 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
