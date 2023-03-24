#[cfg(feature = "IoBusinessOutline")]
use leptos::*;
#[cfg(feature = "IoBusinessOutline")]
///This icon requires the feature `IoBusinessOutline` to be enabled.
#[component]
pub fn BusinessOutline(
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
        "176" y1 = "416" x2 = "176" y2 = "480" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M80,32H272a32,32,0,0,1,32,32V476a4,4,0,0,1-4,4H48a0,0,0,0,1,0,0V64A32,32,0,0,1,80,32Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,192H432a32,32,0,0,1,32,32V480a0,0,0,0,1,0,0H304a0,0,0,0,1,0,0V208A16,16,0,0,1,320,192Z"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M98.08,431.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,431.87Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M98.08,351.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,351.87Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M98.08,271.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,271.87Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M98.08,191.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,191.87Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M98.08,111.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,98.08,111.87Z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M178.08,351.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,351.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M178.08,271.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,271.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M178.08,191.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,191.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M178.08,111.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,178.08,111.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M258.08,431.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,431.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M258.08,351.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,351.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M258.08,271.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,271.87Z" />< ellipse
        xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "176" rx = "15.95" ry =
        "16.03" transform = "translate(-49.47 232.56) rotate(-45)" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M258.08,111.87a16,16,0,1,1,13.79-13.79A16,16,0,0,1,258.08,111.87Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M400,400a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M400,320a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M400,240a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M336,400a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,320a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M336,240a16,16,0,1,0,16,16,16,16,0,0,0-16-16Z"
        /> < title > { title } < / title > < / svg >
    }
}
