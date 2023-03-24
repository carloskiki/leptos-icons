#[cfg(feature = "IoSunnyOutline")]
use leptos::*;
#[cfg(feature = "IoSunnyOutline")]
///This icon requires the feature `IoSunnyOutline` to be enabled.
#[component]
pub fn SunnyOutline(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "256" y1 = "48" x2 = "256" y2 = "96" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "416" x2 = "256" y2
        = "464" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "403.08" y1 = "108.92" x2 =
        "369.14" y2 = "142.86" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "142.86" y1 = "369.14" x2 =
        "108.92" y2 = "403.08" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "464" y1 = "256" x2 = "416" y2
        = "256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "96" y1 = "256" x2 = "48" y2 =
        "256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "403.08" y1 = "403.08" x2 =
        "369.14" y2 = "369.14" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "142.86" y1 = "142.86" x2 =
        "108.92" y2 = "108.92" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "256" r = "80"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
