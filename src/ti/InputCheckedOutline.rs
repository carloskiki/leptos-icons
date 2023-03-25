#[cfg(feature = "TiInputCheckedOutline")]
use leptos::*;
#[cfg(feature = "TiInputCheckedOutline")]
///This icon requires the feature `TiInputCheckedOutline` to be enabled.
#[component]
pub fn InputCheckedOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.885 6.177c-.221-.771-.728-1.409-1.427-1.798-.445-.248-.949-.379-1.457-.379-.862 0-1.661.381-2.219 1h-6.782c-1.654 0-3 1.346-3 3v8c0 1.654 1.346 3 3 3h8c1.654 0 3-1.346 3-3v-6.454l.622-1.088c.39-.7.482-1.51.263-2.281zm-3.758.338c.104-.189.27-.328.459-.416.301-.113.623-.127.9.027.232.13.402.343.476.6.033.117.039.236.03.353-.012.115-.043.227-.092.332l-.021.065-4.006 7.011c-.151.273-.427.461-.742.506l-.132.009c-.267 0-.518-.104-.707-.293l-3-3c-.39-.39-.39-1.024 0-1.414.189-.189.44-.293.707-.293s.518.104.707.293l1.125 1.125.92.92.652-1.125 2.724-4.7zm-.127 10.485h-8c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h6.689l-2.15 3.712-1.125-1.125c-.391-.391-.902-.586-1.414-.586s-1.023.195-1.414.586c-.781.781-.781 2.047 0 2.828l3 3c.378.378.888.586 1.414.586l.277-.02c.621-.087 1.166-.461 1.471-1.01l2.252-3.94v4.969c0 .551-.448 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
