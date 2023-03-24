#[cfg(feature = "BiSolidDryer")]
use leptos::*;
#[cfg(feature = "BiSolidDryer")]
///This icon requires the feature `BiSolidDryer` to be enabled.
#[component]
pub fn Dryer(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M4 22h16a1 1 0 0 0 1-1V5c0-1.654-1.346-3-3-3H6C4.346 2 3 3.346 3 5v16a1 1 0 0 0 1 1zM18 3.924a1 1 0 1 1 0 2 1 1 0 0 1 0-2zm-3 0a1 1 0 1 1 0 2 1 1 0 0 1 0-2zm-3 3.117c3.309 0 6 2.691 6 6s-2.691 6-6 6-6-2.691-6-6 2.691-6 6-6z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.32 12.265c-.415.384-1.041.964-1.041 2.067 0 1.104.626 1.684 1.041 2.068.352.325.4.398.4.6h2c0-1.104-.626-1.684-1.041-2.068-.352-.325-.4-.398-.4-.6s.048-.275.4-.6c.414-.384 1.041-.964 1.041-2.068 0-1.103-.626-1.683-1.041-2.066-.351-.325-.399-.397-.399-.598h-2c0 1.104.627 1.683 1.042 2.066.351.324.399.396.399.597-.001.203-.05.276-.401.602zm4 0c-.414.384-1.04.964-1.04 2.067s.626 1.684 1.04 2.067c.351.325.399.398.399.601h2c0-1.104-.626-1.684-1.04-2.067-.351-.325-.399-.398-.399-.601s.049-.275.399-.601c.414-.384 1.04-.964 1.04-2.068 0-1.103-.626-1.682-1.04-2.065-.35-.324-.399-.397-.399-.598h-2c0 1.103.626 1.683 1.041 2.066.35.324.398.397.398.598.001.202-.048.275-.399.601z"
        /> < title > { title } < / title > < / svg >
    }
}
