#[cfg(feature = "SiMeteor")]
use leptos::*;
#[cfg(feature = "SiMeteor")]
///This icon requires the feature `SiMeteor` to be enabled.
#[component]
pub fn Meteor(
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
        "M0 .234l21.912 20.537s.412.575-.124 1.151c-.535.576-1.236.083-1.236.083L0 .234zm6.508 2.058l17.01 15.638s.413.576-.123 1.152c-.534.576-1.235.083-1.235.083L6.508 2.292zM1.936 6.696l17.01 15.638s.412.576-.123 1.152-1.235.082-1.235.082L1.936 6.696zm10.073-2.635l11.886 10.927s.287.401-.087.805-.863.058-.863.058L12.009 4.061zm-8.567 7.737l11.886 10.926s.285.4-.088.803c-.375.403-.863.059-.863.059L3.442 11.798zm14.187-5.185l5.426 4.955s.142.188-.044.377c-.185.188-.428.027-.428.027l-4.954-5.358v-.001zM6.178 17.231l5.425 4.956s.144.188-.042.377-.427.026-.427.026l-4.956-5.359z"
        /> < title > { title } < / title > < / svg >
    }
}
