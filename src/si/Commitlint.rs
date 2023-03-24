#[cfg(feature = "SiCommitlint")]
use leptos::*;
#[cfg(feature = "SiCommitlint")]
///This icon requires the feature `SiCommitlint` to be enabled.
#[component]
pub fn Commitlint(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.988 4.822v.901h1.845v6.337c0 .809.187 1.433.563 1.875.379.438.913.657 1.6.657h1.458v-.97H12.11c-.37 0-.652-.13-.844-.393-.187-.263-.281-.652-.281-1.17V4.823ZM3.459 7.418c-1.084 0-1.933.325-2.546.976C.304 9.044 0 9.944 0 11.096c0 1.15.304 2.051.913 2.702.613.65 1.462.976 2.546.976a3.67 3.67 0 0 0 1-.137c.33-.088.65-.221.958-.4V13.04c-.271.259-.565.45-.882.576a3 3 0 0 1-1.076.18c-.717 0-1.272-.232-1.664-.7-.387-.47-.582-1.138-.582-2.001 0-.868.195-1.535.582-2.002.388-.467.943-.7 1.664-.7.388 0 .734.06 1.038.181a3.1 3.1 0 0 1 .92.588V7.956a4.255 4.255 0 0 0-.97-.406 3.74 3.74 0 0 0-.988-.132Zm12.855 6.123v1.128H24V13.54zm3.156 2.255-2.302 3.382h1.3l1.694-2.204 1.684 2.204h1.3l-2.301-3.382z"
        /> < title > { title } < / title > < / svg >
    }
}
