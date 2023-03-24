#[cfg(feature = "RiLogosFillZhihu")]
use leptos::*;
#[cfg(feature = "RiLogosFillZhihu")]
///This icon requires the feature `RiLogosFillZhihu` to be enabled.
#[component]
pub fn Zhihu(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M13.373 18.897h1.452l.478 1.637 2.605-1.637h3.07V5.395h-7.605v13.502zM14.918 6.86h4.515v10.57h-1.732l-1.73 1.087-.314-1.084-.739-.003V6.861zm-2.83 4.712H8.846a70.3 70.3 0 0 0 .136-4.56h3.172s.122-1.4-.532-1.384H6.135c.216-.814.488-1.655.813-2.524 0 0-1.493 0-2 1.339-.211.552-.82 2.677-1.904 4.848.365-.04 1.573-.073 2.284-1.378.131-.366.156-.413.318-.902h1.79c0 .651-.074 4.151-.104 4.558h-3.24c-.729 0-.965 1.466-.965 1.466h4.066C6.92 16.131 5.456 18.74 2.8 20.8c1.27.363 2.536-.057 3.162-.614 0 0 1.425-1.297 2.206-4.298l3.346 4.03s.49-1.668-.077-2.481c-.47-.554-1.74-2.052-2.281-2.595l-.907.72c.27-.867.433-1.71.488-2.524h3.822s-.005-1.466-.47-1.466z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
