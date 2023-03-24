#[cfg(feature = "TiEyeOutline")]
use leptos::*;
#[cfg(feature = "TiEyeOutline")]
///This icon requires the feature `TiEyeOutline` to be enabled.
#[component]
pub fn EyeOutline(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9c1.211 0 2.381.355 3.297 1.004 1.301.92 2.43 2.124 3.165 2.996-.735.872-1.864 2.077-3.166 2.996-.915.649-2.085 1.004-3.296 1.004s-2.382-.355-3.299-1.004c-1.301-.92-2.43-2.124-3.164-2.996.734-.872 1.863-2.076 3.164-2.995.917-.65 2.088-1.005 3.299-1.005m0-2c-1.691 0-3.242.516-4.453 1.371-2.619 1.852-4.547 4.629-4.547 4.629s1.928 2.777 4.547 4.629c1.211.855 2.762 1.371 4.453 1.371s3.242-.516 4.451-1.371c2.619-1.852 4.549-4.629 4.549-4.629s-1.93-2.777-4.549-4.629c-1.209-.855-2.76-1.371-4.451-1.371zM12 12c-.553 0-1 .447-1 1 0 .551.447 1 1 1 .551 0 1-.449 1-1 0-.553-.449-1-1-1zM12 16c-1.654 0-3-1.346-3-3s1.346-3 3-3 3 1.346 3 3-1.346 3-3 3zm0-5c-1.104 0-2 .896-2 2s.896 2 2 2 2-.896 2-2-.896-2-2-2z"
        /> < title > { title } < / title > < / svg >
    }
}
