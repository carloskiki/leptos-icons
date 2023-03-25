#[cfg(feature = "RiLogosLineGooglePlay")]
use leptos::*;
#[cfg(feature = "RiLogosLineGooglePlay")]
///This icon requires the feature `RiLogosLineGooglePlay` to be enabled.
#[component]
pub fn GooglePlay(
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
        "M4 1.734a1 1 0 0 1 .501.135l16.004 9.266a1 1 0 0 1 0 1.73L4.501 22.131A1 1 0 0 1 3 21.266V2.734a1 1 0 0 1 1-1zm8.292 11.68l-4.498 4.498 5.699-3.299-1.2-1.2zM5 6.118v11.76l5.88-5.88-5.88-5.88zm10.284 4.302L13.706 12l1.578 1.577L18.008 12l-2.725-1.579zm-7.49-4.336l4.5 4.5 1.199-1.2-5.699-3.3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
