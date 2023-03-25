#[cfg(feature = "RiSystemFillEyeOff")]
use leptos::*;
#[cfg(feature = "RiSystemFillEyeOff")]
///This icon requires the feature `RiSystemFillEyeOff` to be enabled.
#[component]
pub fn EyeOff(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M4.52 5.934L1.393 2.808l1.415-1.415 19.799 19.8-1.415 1.414-3.31-3.31A10.949 10.949 0 0 1 12 21c-5.392 0-9.878-3.88-10.819-9a10.982 10.982 0 0 1 3.34-6.066zm10.237 10.238l-1.464-1.464a3 3 0 0 1-4.001-4.001L7.828 9.243a5 5 0 0 0 6.929 6.929zM7.974 3.76C9.221 3.27 10.58 3 12 3c5.392 0 9.878 3.88 10.819 9a10.947 10.947 0 0 1-2.012 4.592l-3.86-3.86a5 5 0 0 0-5.68-5.68L7.974 3.761z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
