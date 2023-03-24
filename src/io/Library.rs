#[cfg(feature = "IoLibrary")]
use leptos::*;
#[cfg(feature = "IoLibrary")]
///This icon requires the feature `IoLibrary` to be enabled.
#[component]
pub fn Library(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M64,480H48a32,32,0,0,1-32-32V112A32,32,0,0,1,48,80H64a32,32,0,0,1,32,32V448A32,32,0,0,1,64,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,176a32,32,0,0,0-32-32H144a32,32,0,0,0-32,32v28a4,4,0,0,0,4,4H236a4,4,0,0,0,4-4Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M112,448a32,32,0,0,0,32,32h64a32,32,0,0,0,32-32V418a2,2,0,0,0-2-2H114a2,2,0,0,0-2,2Z"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "112" y = "240" width = "128"
        height = "144" rx = "2" ry = "2" />< path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M320,480H288a32,32,0,0,1-32-32V64a32,32,0,0,1,32-32h32a32,32,0,0,1,32,32V448A32,32,0,0,1,320,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M495.89,445.45l-32.23-340c-1.48-15.65-16.94-27-34.53-25.31l-31.85,3c-17.59,1.67-30.65,15.71-29.17,31.36l32.23,340c1.48,15.65,16.94,27,34.53,25.31l31.85-3C484.31,475.14,497.37,461.1,495.89,445.45Z"
        /> < title > { title } < / title > < / svg >
    }
}
