#[cfg(feature = "SiWasmer")]
use leptos::*;
#[cfg(feature = "SiWasmer")]
///This icon requires the feature `SiWasmer` to be enabled.
#[component]
pub fn Wasmer(
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
        "M18.111 3.537c-.011.822-.5 1.208-1.111.86-.611-.353-1.106-1.307-1.111-2.146L12 0v4.651l5.561 3.222.55.32v7.763L22 18.206V5.794l-3.889-2.256Zm-5 3.034c-.011.822-.5 1.208-1.111.86-.611-.352-1.106-1.307-1.111-2.145l-3.89-2.252V7.41l5.562 3.222.55.32v8.038L17 21.241V8.828L13.11 6.57Zm-5 2.759c-.011.822-.5 1.208-1.111.86-.611-.353-1.106-1.307-1.111-2.146L2 5.794v12.413L12 24V11.586L8.111 9.33Z"
        /> < title > { title } < / title > < / svg >
    }
}
