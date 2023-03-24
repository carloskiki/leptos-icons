#[cfg(feature = "SiAlgolia")]
use leptos::*;
#[cfg(feature = "SiAlgolia")]
///This icon requires the feature `SiAlgolia` to be enabled.
#[component]
pub fn Algolia(
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
        "M3.16 0A3.156 3.156 0 000 3.152v17.69A3.161 3.161 0 003.16 24h17.68c1.747 0 3.16-1.42 3.16-3.16V3.16C24 1.413 22.58 0 20.84 0H3.16zm7.87 3.494h2.429A1.04 1.04 0 0114.5 4.535v.822a.14.14 0 01-.174.14 7.679 7.647 0 00-2.043-.276 7.564 7.533 0 00-2.113.297c-.095.021-.182-.045-.182-.14v-.843a1.044 1.04 0 011.041-1.04zm-4.35 2.22a1.04 1.035 0 01.75.306l.414.416c.058.065.051.167-.022.21a7.544 7.513 0 00-.941.801 8 7.967 0 00-.793.932c-.058.065-.153.08-.219.016l-.408-.409a1.043 1.04 0 010-1.472l.496-.494a1.04 1.035 0 01.723-.305zm5.597.35a6.774 6.774 0 016.787 6.778 6.784 6.784 0 01-6.787 6.783c-3.748 0-6.789-3.028-6.789-6.777a6.786 6.786 0 016.79-6.784zm0 2.008a4.783 4.783 0 00-4.783 4.776 4.783 4.783 0 004.783 4.775 4.777 4.777 0 004.784-4.775 4.782 4.782 0 00-4.784-4.776zm.145.838a3.935 3.919 0 013.281 1.988c.036.073.015.16-.057.196l-3.166 1.638c-.093.052-.205-.023-.205-.125V9.05h.002c0-.08.072-.139.145-.139Z"
        /> < title > { title } < / title > < / svg >
    }
}
