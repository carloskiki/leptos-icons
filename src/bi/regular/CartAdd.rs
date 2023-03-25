#[cfg(feature = "BiRegularCartAdd")]
use leptos::*;
#[cfg(feature = "BiRegularCartAdd")]
///This icon requires the feature `BiRegularCartAdd` to be enabled.
#[component]
pub fn CartAdd(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "10.5" cy = "19.5" r = "1.5" /><
        circle xmlns = "http://www.w3.org/2000/svg" cx = "17.5" cy = "19.5" r = "1.5" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 13h2v-2.99h2.99v-2H15V5.03h-2v2.98h-2.99v2H13V13z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 17h8a1 1 0 0 0 .93-.64L21.76 9h-2.14l-2.31 6h-6.64L6.18 4.23A2 2 0 0 0 4.33 3H2v2h2.33l4.75 11.38A1 1 0 0 0 10 17z"
        /> < title > { title } < / title > < / svg >
    }
}
