#[cfg(feature = "TbBrandUbuntu")]
use leptos::*;
#[cfg(feature = "TbBrandUbuntu")]
///This icon requires the feature `TbBrandUbuntu` to be enabled.
#[component]
pub fn BrandUbuntu(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-brand-ubuntu" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 5m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.723 7.41a7.992 7.992 0 0 0 -3.74 -2.162m-3.971 0a7.993 7.993 0 0 0 -3.789 2.216m-1.881 3.215a8 8 0 0 0 -.342 2.32c0 .738 .1 1.453 .287 2.132m1.96 3.428a7.993 7.993 0 0 0 3.759 2.19m4 0a7.993 7.993 0 0 0 3.747 -2.186m1.962 -3.43a8.008 8.008 0 0 0 .287 -2.131c0 -.764 -.107 -1.503 -.307 -2.203"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> <
        title > { title } < / title > < / svg >
    }
}
