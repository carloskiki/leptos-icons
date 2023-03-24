#[cfg(feature = "AiFilledFileWord")]
use leptos::*;
#[cfg(feature = "AiFilledFileWord")]
///This icon requires the feature `AiFilledFileWord` to be enabled.
#[component]
pub fn FileWord(
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
        "M854.6 288.7c6 6 9.4 14.1 9.4 22.6V928c0 17.7-14.3 32-32 32H192c-17.7 0-32-14.3-32-32V96c0-17.7 14.3-32 32-32h424.7c8.5 0 16.7 3.4 22.7 9.4l215.2 215.3zM790.2 326L602 137.8V326h188.2zM512 566.1l52.81 197a12 12 0 0 0 11.6 8.9h31.77a12 12 0 0 0 11.6-8.88l74.37-276a12 12 0 0 0 .4-3.12 12 12 0 0 0-12-12h-35.57a12 12 0 0 0-11.7 9.31l-45.78 199.1-49.76-199.32A12 12 0 0 0 528.1 472h-32.2a12 12 0 0 0-11.64 9.1L434.6 680.01 388.5 481.3a12 12 0 0 0-11.68-9.29h-35.39a12 12 0 0 0-3.11.41 12 12 0 0 0-8.47 14.7l74.17 276A12 12 0 0 0 415.6 772h31.99a12 12 0 0 0 11.59-8.9l52.81-197z"
        /> < title > { title } < / title > < / svg >
    }
}
