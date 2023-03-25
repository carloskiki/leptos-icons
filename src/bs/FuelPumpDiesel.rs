#[cfg(feature = "BsFuelPumpDiesel")]
use leptos::*;
#[cfg(feature = "BsFuelPumpDiesel")]
///This icon requires the feature `BsFuelPumpDiesel` to be enabled.
#[component]
pub fn FuelPumpDiesel(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-fuel-pump-diesel" viewBox = "0 0 16 16" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.5 2a.5.5 0 0 0-.5.5v5a.5.5 0 0 0 .5.5h5a.5.5 0 0 0 .5-.5v-5a.5.5 0 0 0-.5-.5h-5ZM4 14V9h1.796c.5 0 .913.098 1.237.293.325.195.567.479.725.85.161.371.242.82.242 1.344 0 .528-.08.98-.242 1.355a1.805 1.805 0 0 1-.732.861c-.324.198-.734.297-1.23.297H4Zm1.666-4.194h-.692v3.385h.692c.229 0 .427-.035.595-.103a.986.986 0 0 0 .412-.315c.108-.142.188-.318.241-.528.056-.21.083-.456.083-.74 0-.376-.048-.69-.144-.94a1.11 1.11 0 0 0-.436-.569c-.195-.127-.445-.19-.75-.19Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 0a2 2 0 0 0-2 2v13H.5a.5.5 0 0 0 0 1h11a.5.5 0 0 0 0-1H11v-4a1 1 0 0 1 1 1v.5a1.5 1.5 0 0 0 3 0V8h.5a.5.5 0 0 0 .5-.5V4.324c0-.616 0-1.426-.294-2.081a1.969 1.969 0 0 0-.794-.907C14.534 1.111 14.064 1 13.5 1a.5.5 0 0 0 0 1c.436 0 .716.086.9.195a.97.97 0 0 1 .394.458c.147.328.19.746.201 1.222H13.5a.5.5 0 0 0-.5.5V7.5a.5.5 0 0 0 .5.5h.5v4.5a.5.5 0 0 1-1 0V12a2 2 0 0 0-2-2V2a2 2 0 0 0-2-2H3Zm7 2v13H2V2a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1Z"
        /> < title > { title } < / title > < / svg >
    }
}
