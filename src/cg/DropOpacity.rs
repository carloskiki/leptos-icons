#[cfg(feature = "CgDropOpacity")]
use leptos::*;
#[cfg(feature = "CgDropOpacity")]
///This icon requires the feature `CgDropOpacity` to be enabled.
#[component]
pub fn DropOpacity(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.9451 21.9559C12.5884 23.5927 8.42477 23.0167 5.63598 20.2279C2.12126 16.7132 2.12126 11.0147 5.63598 7.49995L11.9999 1.13599L18.3639 7.49995C19.2521 8.38814 19.9158 9.41578 20.3551 10.5124C21.6543 13.7553 20.9906 17.6012 18.364 20.2278C17.6381 20.9538 16.8189 21.5298 15.9451 21.9559ZM7.05019 8.91416L11.9999 3.96441L16.9497 8.91416C18.274 10.2385 18.9568 11.9615 18.998 13.6968H5.00192C5.04309 11.9615 5.72585 10.2385 7.05019 8.91416Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
