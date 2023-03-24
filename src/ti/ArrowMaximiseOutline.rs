#[cfg(feature = "TiArrowMaximiseOutline")]
use leptos::*;
#[cfg(feature = "TiArrowMaximiseOutline")]
///This icon requires the feature `TiArrowMaximiseOutline` to be enabled.
#[component]
pub fn ArrowMaximiseOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 3h-5.243c-1.302 0-2.401.838-2.815 2h-6.942v7.061l.012.12c-1.167.41-2.012 1.512-2.012 2.819v7h7c1.311 0 2.593-.826 3-2h7v-7.061l-.012-.12c1.167-.41 2.012-1.512 2.012-2.819v-7h-2zm-2 15h-5c-.553 0-1-.448-1-1s.447-1 1-1h3v-3.061c0-.552.447-1 1-1s1 .448 1 1v5.061zm-11-11h5.061c.553 0 1 .448 1 1s-.447 1-1 1h-3.061v3.061c0 .552-.448 1-1 1-.553 0-1-.448-1-1v-5.061zm13 3c0 .552-.447 1-1 1s-1-.448-1-1v-1.586l-3.293 3.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.293-3.293h-1.586c-.553 0-1-.448-1-1s.447-1 1-1h5v5zm-10 10h-5v-5c0-.552.447-1 1-1s1 .448 1 1v1.586l3.293-3.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-3.293 3.293h1.586c.553 0 1 .448 1 1s-.448 1-1 1zm2.414-7.414c-.378-.378-.88-.586-1.414-.586-.367 0-.716.105-1.023.289l.023-.228v-2.061h2.061l.229-.023c-.186.307-.29.656-.29 1.023 0 .534.208 1.036.586 1.414s.88.586 1.414.586c.367 0 .716-.105 1.023-.289l-.023.228v2.061h-1.939c-.122 0-.24.015-.356.036.189-.31.295-.664.295-1.036 0-.534-.208-1.036-.586-1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
