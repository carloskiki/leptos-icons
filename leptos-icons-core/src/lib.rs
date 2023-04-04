pub trait IconData<'a> {
    fn data(self) -> &'a Data;
}

pub struct Data {
    pub style: Option<&'static str>,
    pub x: Option<&'static str>,
    pub y: Option<&'static str>,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
    pub view_box: Option<&'static str>,
    pub stroke_linecap: Option<&'static str>,
    pub stroke_linejoin: Option<&'static str>,
    pub stroke_width: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub fill: Option<&'static str>,
    pub data: &'static str,
}

#[allow(non_snake_case)]
pub fn LeptosIconCore(
    cx: leptos::Scope,
    data: &Data,
    width: Option<String>,
    height: Option<String>,
    class: Option<String>,
    style: Option<String>,
) -> leptos::View {
    let mut svg = leptos::svg::svg(cx);
    if let Some(classes) = class {
        svg = svg.classes(classes);
    }
    svg = match (style, data.style) {
        (Some(a), Some(b)) => svg.attr("style", format!("{a} {b}")),
        (Some(a), None) => svg.attr("style", a),
        (None, Some(b)) => svg.attr("style", b),
        (None, None) => svg,
    };
    if let Some(x) = data.x {
        svg = svg.attr("x", x);
    }
    if let Some(y) = data.y {
        svg = svg.attr("x", y);
    }
    svg = match (width, data.width) {
        (Some(a), Some(_b)) => svg.attr("width", a),
        (Some(a), None) => svg.attr("width", a),
        (None, Some(_b)) => svg.attr("width", "1em"),
        (None, None) => svg.attr("width", "1em"),
    };
    svg = match (height, data.height) {
        (Some(a), Some(_b)) => svg.attr("height", a),
        (Some(a), None) => svg.attr("height", a),
        (None, Some(_b)) => svg.attr("height", "1em"),
        (None, None) => svg.attr("height", "1em"),
    };
    if let Some(view_box) = data.view_box {
        svg = svg.attr("viewBox", view_box);
    }
    if let Some(stroke_linecap) = data.stroke_linecap {
        svg = svg.attr("stroke-linecap", stroke_linecap);
    }
    if let Some(stroke_linejoin) = data.stroke_linejoin {
        svg = svg.attr("stroke-linejoin", stroke_linejoin);
    }
    if let Some(stroke_width) = data.stroke_width {
        svg = svg.attr("stroke-width", stroke_width);
    }
    if let Some(stroke) = data.stroke {
        svg = svg.attr("stroke", stroke);
    }
    svg = svg.attr("fill", data.fill.unwrap_or("currentColor"));
    svg = svg.attr("role", "graphics-symbol");
    svg = svg.inner_html(data.data);
    leptos::IntoView::into_view(svg, cx)
}
