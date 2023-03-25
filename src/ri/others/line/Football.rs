#[cfg(feature = "RiOthersLineFootball")]
use leptos::*;
#[cfg(feature = "RiOthersLineFootball")]
///This icon requires the feature `RiOthersLineFootball` to be enabled.
#[component]
pub fn Football(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2zm1.67 14h-3.34l-1.38 1.897.554 1.706A7.993 7.993 0 0 0 12 20c.871 0 1.71-.14 2.496-.397l.553-1.706L13.669 16zm-8.376-5.128l-1.292.937L4 12c0 1.73.549 3.331 1.482 4.64h1.91l1.323-1.82-1.028-3.17-2.393-.778zm13.412 0l-2.393.778-1.028 3.17 1.322 1.82h1.91A7.964 7.964 0 0 0 20 12l-.003-.19-1.291-.938zM12 9.536l-2.344 1.702.896 2.762h2.895l.896-2.762L12 9.536zm2.291-5.203L13 5.273V7.79l2.694 1.957 2.239-.727.554-1.703a8.014 8.014 0 0 0-4.196-2.984zm-4.583 0a8.014 8.014 0 0 0-4.195 2.985l.554 1.702 2.239.727L11 7.79V5.273l-1.292-.94z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
