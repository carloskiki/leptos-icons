#[cfg(feature = "TbBooksOff")]
use leptos::*;
#[cfg(feature = "TbBooksOff")]
///This icon requires the feature `TbBooksOff` to be enabled.
#[component]
pub fn BooksOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-books-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 9v10a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-14" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 4a1 1 0 0 1 1 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 5a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v4" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M13 13v6a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 8h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 16h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.254 10.244l-1.218 -4.424a1.02 1.02 0 0 1 .634 -1.219l.133 -.041l2.184 -.53c.562 -.135 1.133 .19 1.282 .732l3.236 11.75"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.585 19.589l-1.572 .38c-.562 .136 -1.133 -.19 -1.282 -.731l-.952 -3.458" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M14 9l4 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.207 15.199l.716 -.18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
