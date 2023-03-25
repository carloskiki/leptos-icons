#[cfg(feature = "BiSolidFlame")]
use leptos::*;
#[cfg(feature = "BiSolidFlame")]
///This icon requires the feature `BiSolidFlame` to be enabled.
#[component]
pub fn Flame(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12.579 2.393a.982.982 0 0 0-1.153.006C9.592 3.728 4 8.252 4 14c0 3.247 1.948 6.043 4.734 7.296A3.971 3.971 0 0 1 8 19c-.017-3.221 3.558-6.893 3.71-7a.497.497 0 0 1 .579 0c.152.107 3.711 2.974 3.711 7.002 0 .854-.275 1.643-.733 2.294C18.052 20.043 20 17.248 20 14.005c0-5.861-5.582-10.307-7.421-11.612z"
        /> < title > { title } < / title > < / svg >
    }
}
