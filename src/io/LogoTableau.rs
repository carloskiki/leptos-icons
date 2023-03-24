#[cfg(feature = "IoLogoTableau")]
use leptos::*;
#[cfg(feature = "IoLogoTableau")]
///This icon requires the feature `IoLogoTableau` to be enabled.
#[component]
pub fn LogoTableau(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M242.69,340.3h26.62V267.7h67V241.88h-67v-72.6H242.69v72.6H176.54V267.7h66.15Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M119.26,445.18h22.59V380.64h59.7V360.47h-59.7V295.13H119.26v65.34H59.56v20.17h59.7Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M370.15,212h22.59V147.5h60.5V128.13h-60.5V62.79H370.15v65.34h-59.7V147.5h59.7Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M246.72,496h19.36V450h41.15V433.08H266.08v-46H246.72v46H206.39V450h40.33Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M120.07,212h21V146.69h60.51V128.13H141V62.79h-21v65.34H59.56v18.56h60.51Z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M435.72,308.84h19.36V263.66H496V245.92H455.08V200.74H435.72v45.18H395.39v17.74h40.33Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" d =
        "M370.15,445.18h22.59V380.64h60.5V360.47h-60.5V295.13H370.15v65.34h-59.7v20.17h59.7Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M307,74.08V60.37H266.66V16H252.14V60.37H211.81V74.08h40.33v44.37h14.52V74.08ZM56.11,305.61H70.63V261.24H111V247.53H70.63V204H56.11v43.56H16v14.52L56.11,262Z"
        /> < title > { title } < / title > < / svg >
    }
}
