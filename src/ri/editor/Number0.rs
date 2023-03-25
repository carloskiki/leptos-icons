#[cfg(feature = "RiEditorNumber0")]
use leptos::*;
#[cfg(feature = "RiEditorNumber0")]
///This icon requires the feature `RiEditorNumber0` to be enabled.
#[component]
pub fn Number0(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M12 1.5c1.321 0 2.484.348 3.447.994.963.645 1.726 1.588 2.249 2.778.522 1.19.804 2.625.804 4.257v4.942c0 1.632-.282 3.068-.804 4.257-.523 1.19-1.286 2.133-2.25 2.778-.962.646-2.125.994-3.446.994-1.321 0-2.484-.348-3.447-.994-.963-.645-1.726-1.588-2.249-2.778-.522-1.19-.804-2.625-.804-4.257V9.529c0-1.632.282-3.068.804-4.257.523-1.19 1.286-2.133 2.25-2.778C9.515 1.848 10.678 1.5 12 1.5zm0 2c-.916 0-1.694.226-2.333.655-.637.427-1.158 1.07-1.532 1.92-.412.94-.635 2.108-.635 3.454v4.942c0 1.346.223 2.514.635 3.453.374.851.895 1.494 1.532 1.921.639.429 1.417.655 2.333.655.916 0 1.694-.226 2.333-.655.637-.427 1.158-1.07 1.532-1.92.412-.94.635-2.108.635-3.454V9.529c0-1.346-.223-2.514-.635-3.453-.374-.851-.895-1.494-1.532-1.921C13.694 3.726 12.916 3.5 12 3.5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
