#[cfg(feature = "IoPhonePortrait")]
use leptos::*;
#[cfg(feature = "IoPhonePortrait")]
///This icon requires the feature `IoPhonePortrait` to be enabled.
#[component]
pub fn PhonePortrait(
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
        "M336,0H176a64,64,0,0,0-64,64V448a64,64,0,0,0,64,64H336a64,64,0,0,0,64-64V64A64,64,0,0,0,336,0Zm32,448a32,32,0,0,1-32,32H176a32,32,0,0,1-32-32V64a32,32,0,0,1,32-32h11.35a7.94,7.94,0,0,1,7.3,4.75A32,32,0,0,0,224,56h64a32,32,0,0,0,29.35-19.25,7.94,7.94,0,0,1,7.3-4.75H336a32,32,0,0,1,32,32Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,48h0a11.88,11.88,0,0,0-9.53,4.69A48,48,0,0,1,288,72H224a48,48,0,0,1-38.47-19.31A11.88,11.88,0,0,0,176,48h0a16,16,0,0,0-16,16V448a16,16,0,0,0,16,16H336a16,16,0,0,0,16-16V64A16,16,0,0,0,336,48Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,0H176a64,64,0,0,0-64,64V448a64,64,0,0,0,64,64H336a64,64,0,0,0,64-64V64A64,64,0,0,0,336,0Zm32,448a32,32,0,0,1-32,32H176a32,32,0,0,1-32-32V64a32,32,0,0,1,32-32h11.35a7.94,7.94,0,0,1,7.3,4.75A32,32,0,0,0,224,56h64a32,32,0,0,0,29.35-19.25,7.94,7.94,0,0,1,7.3-4.75H336a32,32,0,0,1,32,32Z"
        /> < title > { title } < / title > < / svg >
    }
}
