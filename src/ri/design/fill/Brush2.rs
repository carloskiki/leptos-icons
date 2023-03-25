#[cfg(feature = "RiDesignFillBrush2")]
use leptos::*;
#[cfg(feature = "RiDesignFillBrush2")]
///This icon requires the feature `RiDesignFillBrush2` to be enabled.
#[component]
pub fn Brush2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M16.536 15.95l2.12-2.122-3.181-3.182 3.535-3.535-2.12-2.121-3.536 3.535-3.182-3.182L8.05 7.464l8.486 8.486zM13.354 5.697l2.828-2.829a1 1 0 0 1 1.414 0l3.536 3.536a1 1 0 0 1 0 1.414l-2.829 2.828 2.475 2.475a1 1 0 0 1 0 1.415L13 22.314a1 1 0 0 1-1.414 0l-9.9-9.9a1 1 0 0 1 0-1.414l7.778-7.778a1 1 0 0 1 1.415 0l2.475 2.475z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
