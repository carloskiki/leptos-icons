#[cfg(feature = "RiDesignLineBrush")]
use leptos::*;
#[cfg(feature = "RiDesignLineBrush")]
///This icon requires the feature `RiDesignLineBrush` to be enabled.
#[component]
pub fn Brush(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M15.456 9.678l-.142-.142a5.475 5.475 0 0 0-2.39-1.349c-2.907-.778-5.699.869-6.492 3.83-.043.16-.066.34-.104.791-.154 1.87-.594 3.265-1.8 4.68 2.26.888 4.938 1.514 6.974 1.514a5.505 5.505 0 0 0 5.31-4.078 5.497 5.497 0 0 0-1.356-5.246zM13.29 6.216l4.939-3.841a1 1 0 0 1 1.32.082l2.995 2.994a1 1 0 0 1 .082 1.321l-3.84 4.938a7.505 7.505 0 0 1-7.283 9.292C8 21.002 3.5 19.5 1 18c3.98-3 3.047-4.81 3.5-6.5 1.058-3.95 4.842-6.257 8.789-5.284zm3.413 1.879c.065.063.13.128.193.194l1.135 1.134 2.475-3.182-1.746-1.746-3.182 2.475 1.125 1.125z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
