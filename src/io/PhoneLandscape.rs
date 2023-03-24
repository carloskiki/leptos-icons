#[cfg(feature = "IoPhoneLandscape")]
use leptos::*;
#[cfg(feature = "IoPhoneLandscape")]
///This icon requires the feature `IoPhoneLandscape` to be enabled.
#[component]
pub fn PhoneLandscape(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M0,176V336a64,64,0,0,0,64,64H448a64,64,0,0,0,64-64V176a64,64,0,0,0-64-64H64A64,64,0,0,0,0,176Zm448-32a32,32,0,0,1,32,32V336a32,32,0,0,1-32,32H64a32,32,0,0,1-32-32V324.65a7.94,7.94,0,0,1,4.75-7.3A32,32,0,0,0,56,288V224a32,32,0,0,0-19.25-29.35,7.94,7.94,0,0,1-4.75-7.3V176a32,32,0,0,1,32-32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M48,176h0a11.88,11.88,0,0,0,4.69,9.53A48,48,0,0,1,72,224v64a48,48,0,0,1-19.31,38.47A11.88,11.88,0,0,0,48,336h0a16,16,0,0,0,16,16H448a16,16,0,0,0,16-16V176a16,16,0,0,0-16-16H64A16,16,0,0,0,48,176Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M0,176V336a64,64,0,0,0,64,64H448a64,64,0,0,0,64-64V176a64,64,0,0,0-64-64H64A64,64,0,0,0,0,176Zm448-32a32,32,0,0,1,32,32V336a32,32,0,0,1-32,32H64a32,32,0,0,1-32-32V324.65a7.94,7.94,0,0,1,4.75-7.3A32,32,0,0,0,56,288V224a32,32,0,0,0-19.25-29.35,7.94,7.94,0,0,1-4.75-7.3V176a32,32,0,0,1,32-32Z"
        /> < title > { title } < / title > < / svg >
    }
}
