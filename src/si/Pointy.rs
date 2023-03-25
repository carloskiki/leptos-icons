#[cfg(feature = "SiPointy")]
use leptos::*;
#[cfg(feature = "SiPointy")]
///This icon requires the feature `SiPointy` to be enabled.
#[component]
pub fn Pointy(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.076.025S4.52.234 2.833 2.751c-1.58 2.351-1.465 5.145-1.1 8.121C2.096 13.831 2.587 24 2.587 24c.002.003 11.235-11.526 11.23-11.506 1.75-1.805 2.408-4.468 2.395-5.961-.037-4.274-3.461-6.815-8.136-6.508zm.777 10.774c-1.991 0-3.604-1.632-3.604-3.645 0-2.015 1.614-3.649 3.604-3.649s3.642 1.512 3.642 3.527c0 2.011-1.652 3.767-3.642 3.767zm2.765-3.741a1.58 1.58 0 1 1-3.162 0 1.58 1.58 0 0 1 3.162 0zm10.879 1.431s-2.325.158-3.644.57c-1.317.413-2.502 1.076-2.502 1.076s.495-.852.705-2.361c.207-1.511-.14-2.652-.14-2.652l5.581 3.367Z"
        /> < title > { title } < / title > < / svg >
    }
}
