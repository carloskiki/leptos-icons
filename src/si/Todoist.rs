#[cfg(feature = "SiTodoist")]
use leptos::*;
#[cfg(feature = "SiTodoist")]
///This icon requires the feature `SiTodoist` to be enabled.
#[component]
pub fn Todoist(
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
        "M21 0H3C1.35 0 0 1.35 0 3v3.858s3.854 2.24 4.098 2.38c.31.18.694.177 1.004 0 .26-.147 8.02-4.608 8.136-4.675.279-.161.58-.107.748-.01.164.097.606.348.84.48.232.134.221.502.013.622l-9.712 5.59c-.346.2-.69.204-1.048.002C3.478 10.907.998 9.463 0 8.882v2.02l4.098 2.38c.31.18.694.177 1.004 0 .26-.147 8.02-4.609 8.136-4.676.279-.16.58-.106.748-.008.164.096.606.347.84.48.232.133.221.5.013.62-.208.121-9.288 5.346-9.712 5.59-.346.2-.69.205-1.048.002C3.478 14.951.998 13.506 0 12.926v2.02l4.098 2.38c.31.18.694.177 1.004 0 .26-.147 8.02-4.609 8.136-4.676.279-.16.58-.106.748-.009.164.097.606.348.84.48.232.133.221.502.013.622l-9.712 5.59c-.346.199-.69.204-1.048.001C3.478 18.994.998 17.55 0 16.97V21c0 1.65 1.35 3 3 3h18c1.65 0 3-1.35 3-3V3c0-1.65-1.35-3-3-3z"
        /> < title > { title } < / title > < / svg >
    }
}
