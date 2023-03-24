#[cfg(feature = "TiThSmallOutline")]
use leptos::*;
#[cfg(feature = "TiThSmallOutline")]
///This icon requires the feature `TiThSmallOutline` to be enabled.
#[component]
pub fn ThSmallOutline(
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
        "M6 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM6 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM13 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 16h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 9h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2zM20 2h-2c-1.104 0-2 .896-2 2v2c0 1.104.896 2 2 2h2c1.104 0 2-.896 2-2v-2c0-1.104-.896-2-2-2zm0 4h-2v-2h2v2z"
        /> < title > { title } < / title > < / svg >
    }
}
