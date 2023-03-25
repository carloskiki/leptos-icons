#[cfg(feature = "TiHeartOutline")]
use leptos::*;
#[cfg(feature = "TiHeartOutline")]
///This icon requires the feature `TiHeartOutline` to be enabled.
#[component]
pub fn HeartOutline(
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
        "M12 20c-.195 0-.391-.057-.561-.172-.225-.151-5.508-3.73-7.146-5.371-1.831-1.831-2.043-3.777-2.043-5.082 0-2.964 2.411-5.375 5.375-5.375 1.802 0 3.398.891 4.375 2.256.977-1.365 2.573-2.256 4.375-2.256 2.964 0 5.375 2.411 5.375 5.375 0 1.305-.212 3.251-2.043 5.082-1.641 1.641-6.923 5.22-7.146 5.371-.17.115-.366.172-.561.172zm-4.375-14c-1.861 0-3.375 1.514-3.375 3.375 0 1.093.173 2.384 1.457 3.668 1.212 1.212 4.883 3.775 6.293 4.746 1.41-.971 5.081-3.534 6.293-4.746 1.284-1.284 1.457-2.575 1.457-3.668 0-1.861-1.514-3.375-3.375-3.375s-3.375 1.514-3.375 3.375c0 .552-.447 1-1 1s-1-.448-1-1c0-1.861-1.514-3.375-3.375-3.375z"
        /> < title > { title } < / title > < / svg >
    }
}
