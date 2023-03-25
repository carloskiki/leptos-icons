#[cfg(feature = "TbDatabaseExport")]
use leptos::*;
#[cfg(feature = "TbDatabaseExport")]
///This icon requires the feature `TbDatabaseExport` to be enabled.
#[component]
pub fn DatabaseExport(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-database-export" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 6m-8 0a8 3 0 1 0 16 0a8 3 0 1 0 -16 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 6v6c0 1.657 3.582 3 8 3a19.84 19.84 0 0 0 3.302 -.267m4.698 -2.733v-6" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 12v6c0 1.599 3.335 2.905 7.538 3m8.462 -7v-2m-6 7h7m-3 -3l3 3l-3 3" /> <
        title > { title } < / title > < / svg >
    }
}
