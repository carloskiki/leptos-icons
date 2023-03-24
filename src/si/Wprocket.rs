#[cfg(feature = "SiWprocket")]
use leptos::*;
#[cfg(feature = "SiWprocket")]
///This icon requires the feature `SiWprocket` to be enabled.
#[component]
pub fn Wprocket(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.723.666c-.08-.276.08-.47.356-.47h2.283c.16 0 .31.137.356.274L8.393 7.07h.08L11.491.218A.374.374 0 0111.824 0h.356c.172 0 .287.092.333.218l3.018 6.85h.08L17.286.47a.397.397 0 01.356-.275h2.284c.275 0 .424.195.355.47l-3.683 13.082a.369.369 0 01-.356.275h-.31a.38.38 0 01-.333-.218l-3.568-7.963h-.058l-3.545 7.963a.403.403 0 01-.333.218h-.31a.379.379 0 01-.356-.275L3.723.666m8.308 7.917l-2.594 5.818a1.663 1.663 0 01-.344.448v.004a1.466 1.466 0 01-.688.34l1.4 8.687c.091.16.263.16.367 0l1.79-2.72 1.64 2.708c.104.16.265.16.368 0l1.584-8.698a1.5 1.5 0 01-.832-.618l-.02-.03a1.405 1.405 0 01-.066-.12l-.609-1.366h-.003Z"
        /> < title > { title } < / title > < / svg >
    }
}
