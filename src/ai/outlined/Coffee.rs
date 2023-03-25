#[cfg(feature = "AiOutlinedCoffee")]
use leptos::*;
#[cfg(feature = "AiOutlinedCoffee")]
///This icon requires the feature `AiOutlinedCoffee` to be enabled.
#[component]
pub fn Coffee(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M275 281c19.9 0 36-16.1 36-36V36c0-19.9-16.1-36-36-36s-36 16.1-36 36v209c0 19.9 16.1 36 36 36zm613 144H768c0-39.8-32.2-72-72-72H200c-39.8 0-72 32.2-72 72v248c0 3.4.2 6.7.7 9.9-.5 7-.7 14-.7 21.1 0 176.7 143.3 320 320 320 160.1 0 292.7-117.5 316.3-271H888c39.8 0 72-32.2 72-72V497c0-39.8-32.2-72-72-72zM696 681h-1.1c.7 7.6 1.1 15.2 1.1 23 0 137-111 248-248 248S200 841 200 704c0-7.8.4-15.4 1.1-23H200V425h496v256zm192-8H776V497h112v176zM613 281c19.9 0 36-16.1 36-36V36c0-19.9-16.1-36-36-36s-36 16.1-36 36v209c0 19.9 16.1 36 36 36zm-170 0c19.9 0 36-16.1 36-36V36c0-19.9-16.1-36-36-36s-36 16.1-36 36v209c0 19.9 16.1 36 36 36z"
        /> < title > { title } < / title > < / svg >
    }
}
