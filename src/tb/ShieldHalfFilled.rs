#[cfg(feature = "TbShieldHalfFilled")]
use leptos::*;
#[cfg(feature = "TbShieldHalfFilled")]
///This icon requires the feature `TbShieldHalfFilled` to be enabled.
#[component]
pub fn ShieldHalfFilled(
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
        "icon icon-tabler icon-tabler-shield-half-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3a12 12 0 0 0 8.5 3a12 12 0 0 1 -8.5 15a12 12 0 0 1 -8.5 -15a12 12 0 0 0 8.5 -3"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 3v18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 11h8.9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 8h8.9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5h3.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 17h6.2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 14h8" /> < title > { title } < / title > <
        / svg >
    }
}
