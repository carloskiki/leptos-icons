#[cfg(feature = "HiLgSolidStar")]
use leptos::*;
#[cfg(feature = "HiLgSolidStar")]
///This icon requires the feature `HiLgSolidStar` to be enabled.
#[component]
pub fn Star(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10.7881 3.2108C11.2364 2.13286 12.7635 2.13286 13.2118 3.2108L15.2938 8.21652L20.6979 8.64976C21.8616 8.74306 22.3335 10.1953 21.4469 10.9549L17.3295 14.4818L18.5874 19.7553C18.8583 20.8909 17.6229 21.7884 16.6266 21.1799L11.9999 18.354L7.37329 21.1799C6.37697 21.7884 5.14158 20.8909 5.41246 19.7553L6.67038 14.4818L2.55303 10.9549C1.66639 10.1953 2.13826 8.74306 3.302 8.64976L8.70609 8.21652L10.7881 3.2108Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
