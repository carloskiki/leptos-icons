#[cfg(feature = "SiHackclub")]
use leptos::*;
#[cfg(feature = "SiHackclub")]
///This icon requires the feature `SiHackclub` to be enabled.
#[component]
pub fn Hackclub(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0C2.4 0 0 2.4 0 12s2.4 12 12 12 12-2.4 12-12S21.6 0 12 0zm4.5 19.5094h-3.3094V13.003c0-.975-.1875-1.6218-.8343-1.6218-.7125 0-1.575 1.003-1.575 2.625v5.503H7.5V4.9689l3.2906-.5625v5.428c.7125-.6468 1.7063-.928 2.7188-.928 2.1562 0 2.9906 1.4156 2.9906 3.628z"
        /> < title > { title } < / title > < / svg >
    }
}
