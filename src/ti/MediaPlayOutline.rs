#[cfg(feature = "TiMediaPlayOutline")]
use leptos::*;
#[cfg(feature = "TiMediaPlayOutline")]
///This icon requires the feature `TiMediaPlayOutline` to be enabled.
#[component]
pub fn MediaPlayOutline(
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
        "M8.998 7.002l.085.078 5.051 4.92-5.096 4.964-.038.036-.002-9.998m.002-2.002c-1.104 0-2 .896-2 2v10c0 1.104.896 2 2 2 .543 0 1.033-.218 1.393-.568 2.644-2.573 6.607-6.432 6.607-6.432s-3.963-3.859-6.604-6.433c-.363-.349-.853-.567-1.396-.567z"
        /> < title > { title } < / title > < / svg >
    }
}
