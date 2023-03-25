#[cfg(feature = "TbHomeQuestion")]
use leptos::*;
#[cfg(feature = "TbHomeQuestion")]
///This icon requires the feature `TbHomeQuestion` to be enabled.
#[component]
pub fn HomeQuestion(
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
        "icon icon-tabler icon-tabler-home-question" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.136 11.136l-8.136 -8.136l-9 9h2v7a2 2 0 0 0 2 2h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 21v-6a2 2 0 0 1 2 -2h2c.467 0 .896 .16 1.236 .428" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 22v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 19a2 2 0 0 0 .914 -3.782a1.98 1.98 0 0 0 -2.414 .483" /> < title > { title }
        < / title > < / svg >
    }
}
