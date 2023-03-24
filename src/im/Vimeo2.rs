#[cfg(feature = "ImVimeo2")]
use leptos::*;
#[cfg(feature = "ImVimeo2")]
///This icon requires the feature `ImVimeo2` to be enabled.
#[component]
pub fn Vimeo2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.5 0h-13c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h13c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM13.463 5.313c-0.050 1.125-0.838 2.666-2.359 4.622-1.572 2.044-2.903 3.066-3.991 3.066-0.675 0-1.244-0.622-1.709-1.866-0.313-1.141-0.622-2.281-0.934-3.422-0.344-1.244-0.716-1.866-1.112-1.866-0.087 0-0.391 0.181-0.906 0.544l-0.544-0.7c0.572-0.5 1.134-1.003 1.687-1.503 0.763-0.656 1.331-1.003 1.712-1.038 0.9-0.087 1.453 0.528 1.662 1.844 0.225 1.422 0.381 2.303 0.469 2.65 0.259 1.178 0.544 1.766 0.856 1.766 0.241 0 0.606-0.381 1.091-1.147s0.744-1.347 0.778-1.747c0.069-0.659-0.191-0.991-0.778-0.991-0.278 0-0.563 0.063-0.856 0.191 0.569-1.859 1.653-2.766 3.256-2.712 1.188 0.034 1.747 0.803 1.678 2.309z"
        /> < title > { title } < / title > < / svg >
    }
}
