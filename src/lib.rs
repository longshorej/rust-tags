pub mod rustags {
    pub mod core {
        #[derive(Clone)]
        pub struct Fragment {
            pub data: String,
            element: bool
        }

        impl<'a> From<&'a str> for Fragment {
            fn from(item: &'a str) -> Self {
                Fragment { data: escape_text(item), element: true }
            }
        }

        impl From<i32> for Fragment {
            fn from(item: i32) -> Self {
                Fragment { data: item.to_string(), element: true }
            }
        }

        fn escape_tag_name(name: &str) -> String {
            // @TODO
            name.to_string()
        }

        fn escape_attribute_name(value: &str) -> String {
            // @TODO
            value.to_string()
        }

        fn escape_attribute_value(value: &str) -> String {
            // @TODO
            value.to_string()
        }

        fn escape_text(text: &str) -> String {
            text
                .chars()
                .map(|c| match c {
                    '<'  => format!("&lt;"),
                    '>'  => format!("&gt;"),
                    '"'  => format!("&quot;"),
                    '\'' => format!("&pos;"),
                    '&'  => format!("&amp;"),
                    _    => format!("{}", c),

                })
                .collect()
        }

        pub fn attribute(name: &str, value: &str) -> Fragment {
            Fragment {
                data: format!("{0}=\"{1}\"", escape_attribute_name(name), escape_attribute_value(value)),
                element: false
            }
        }

        pub fn empty_attribute(name: &str) -> Fragment {
            Fragment {
                data: format!("{0}", escape_attribute_name(name)),
                element: false
            }
        }

        pub fn tag(name: &str, unary: bool, children: Vec<Fragment>) -> Fragment {
            // @TODO way to partition here?

            let elements: Vec<String> =
                children
                  .iter()
                  .filter(|c| c.element)
                  .map(|c| c.data.clone())
                  .collect();

            let attributes: Vec<String> =
                children
                  .iter()
                  .filter(|c| !c.element)
                  .map(|c| c.data.clone())
                  .collect();

            let elements_data = elements.join("");

            let attributes_data =
                if attributes.is_empty() {
                    format!("")
                } else {
                    format!(" {}", attributes.join(" "))
                };

            let data =
                if unary {
                    format!("<{0}{1}/>", escape_tag_name(name), attributes_data)
                } else {
                    format!("<{0}{1}>{2}</{0}>", escape_tag_name(name), attributes_data, elements_data)
                };

            Fragment { data: data, element: true }
        }

    }

    pub mod tags {
        use rustags::core::*;

        // Unary Tags
        pub fn br() -> Fragment { tag("br", true, vec![]) }
        pub fn hr(children: &[Fragment]) -> Fragment { tag("hr", true, children.to_vec()) }

        // Binary tags
        pub fn a(children: &[Fragment]) -> Fragment { tag("a", false, children.to_vec()) }
        pub fn abbr(children: &[Fragment]) -> Fragment { tag("abbr", false, children.to_vec()) }
        pub fn acronym(children: &[Fragment]) -> Fragment { tag("acronym", false, children.to_vec()) }
        pub fn address(children: &[Fragment]) -> Fragment { tag("address", false, children.to_vec()) }
        pub fn applet(children: &[Fragment]) -> Fragment { tag("applet", false, children.to_vec()) }
        pub fn area(children: &[Fragment]) -> Fragment { tag("area", false, children.to_vec()) }
        pub fn article(children: &[Fragment]) -> Fragment { tag("article", false, children.to_vec()) }
        pub fn aside(children: &[Fragment]) -> Fragment { tag("aside", false, children.to_vec()) }
        pub fn audio(children: &[Fragment]) -> Fragment { tag("audio", false, children.to_vec()) }
        pub fn b(children: &[Fragment]) -> Fragment { tag("b", false, children.to_vec()) }
        pub fn base(children: &[Fragment]) -> Fragment { tag("base", false, children.to_vec()) }
        pub fn basefont(children: &[Fragment]) -> Fragment { tag("basefont", false, children.to_vec()) }
        pub fn bdi(children: &[Fragment]) -> Fragment { tag("bdi", false, children.to_vec()) }
        pub fn bdo(children: &[Fragment]) -> Fragment { tag("bdo", false, children.to_vec()) }
        pub fn big(children: &[Fragment]) -> Fragment { tag("big", false, children.to_vec()) }
        pub fn blockquote(children: &[Fragment]) -> Fragment { tag("blockquote", false, children.to_vec()) }
        pub fn body(children: &[Fragment]) -> Fragment { tag("body", false, children.to_vec()) }
        pub fn button(children: &[Fragment]) -> Fragment { tag("button", false, children.to_vec()) }
        pub fn canvas(children: &[Fragment]) -> Fragment { tag("canvas", false, children.to_vec()) }
        pub fn caption(children: &[Fragment]) -> Fragment { tag("caption", false, children.to_vec()) }
        pub fn center(children: &[Fragment]) -> Fragment { tag("center", false, children.to_vec()) }
        pub fn cite(children: &[Fragment]) -> Fragment { tag("cite", false, children.to_vec()) }
        pub fn code(children: &[Fragment]) -> Fragment { tag("code", false, children.to_vec()) }
        pub fn col(children: &[Fragment]) -> Fragment { tag("col", false, children.to_vec()) }
        pub fn colgroup(children: &[Fragment]) -> Fragment { tag("colgroup", false, children.to_vec()) }
        pub fn datalist(children: &[Fragment]) -> Fragment { tag("datalist", false, children.to_vec()) }
        pub fn dd(children: &[Fragment]) -> Fragment { tag("dd", false, children.to_vec()) }
        pub fn del(children: &[Fragment]) -> Fragment { tag("del", false, children.to_vec()) }
        pub fn details(children: &[Fragment]) -> Fragment { tag("details", false, children.to_vec()) }
        pub fn dfn(children: &[Fragment]) -> Fragment { tag("dfn", false, children.to_vec()) }
        pub fn dialog(children: &[Fragment]) -> Fragment { tag("dialog", false, children.to_vec()) }
        pub fn dir(children: &[Fragment]) -> Fragment { tag("dir", false, children.to_vec()) }
        pub fn div(children: &[Fragment]) -> Fragment { tag("div", false, children.to_vec()) }
        pub fn dl(children: &[Fragment]) -> Fragment { tag("dl", false, children.to_vec()) }
        pub fn dt(children: &[Fragment]) -> Fragment { tag("dt", false, children.to_vec()) }
        pub fn em(children: &[Fragment]) -> Fragment { tag("em", false, children.to_vec()) }
        pub fn embed(children: &[Fragment]) -> Fragment { tag("embed", false, children.to_vec()) }
        pub fn fieldset(children: &[Fragment]) -> Fragment { tag("fieldset", false, children.to_vec()) }
        pub fn figcaption(children: &[Fragment]) -> Fragment { tag("figcaption", false, children.to_vec()) }
        pub fn figure(children: &[Fragment]) -> Fragment { tag("figure", false, children.to_vec()) }
        pub fn font(children: &[Fragment]) -> Fragment { tag("font", false, children.to_vec()) }
        pub fn footer(children: &[Fragment]) -> Fragment { tag("footer", false, children.to_vec()) }
        pub fn form(children: &[Fragment]) -> Fragment { tag("form", false, children.to_vec()) }
        pub fn frame(children: &[Fragment]) -> Fragment { tag("frame", false, children.to_vec()) }
        pub fn frameset(children: &[Fragment]) -> Fragment { tag("frameset", false, children.to_vec()) }
        pub fn h1(children: &[Fragment]) -> Fragment { tag("h1", false, children.to_vec()) }
        pub fn head(children: &[Fragment]) -> Fragment { tag("head", false, children.to_vec()) }
        pub fn header(children: &[Fragment]) -> Fragment { tag("header", false, children.to_vec()) }
        pub fn html(children: &[Fragment]) -> Fragment { tag("html", false, children.to_vec()) }
        pub fn i(children: &[Fragment]) -> Fragment { tag("i", false, children.to_vec()) }
        pub fn iframe(children: &[Fragment]) -> Fragment { tag("iframe", false, children.to_vec()) }
        pub fn img(children: &[Fragment]) -> Fragment { tag("img", false, children.to_vec()) }
        pub fn input(children: &[Fragment]) -> Fragment { tag("input", false, children.to_vec()) }
        pub fn ins(children: &[Fragment]) -> Fragment { tag("ins", false, children.to_vec()) }
        pub fn kbd(children: &[Fragment]) -> Fragment { tag("kbd", false, children.to_vec()) }
        pub fn label(children: &[Fragment]) -> Fragment { tag("label", false, children.to_vec()) }
        pub fn legend(children: &[Fragment]) -> Fragment { tag("legend", false, children.to_vec()) }
        pub fn li(children: &[Fragment]) -> Fragment { tag("li", false, children.to_vec()) }
        pub fn link(children: &[Fragment]) -> Fragment { tag("link", false, children.to_vec()) }
        pub fn main(children: &[Fragment]) -> Fragment { tag("main", false, children.to_vec()) }
        pub fn map(children: &[Fragment]) -> Fragment { tag("map", false, children.to_vec()) }
        pub fn mark(children: &[Fragment]) -> Fragment { tag("mark", false, children.to_vec()) }
        pub fn menu(children: &[Fragment]) -> Fragment { tag("menu", false, children.to_vec()) }
        pub fn menuitem(children: &[Fragment]) -> Fragment { tag("menuitem", false, children.to_vec()) }
        pub fn meta(children: &[Fragment]) -> Fragment { tag("meta", false, children.to_vec()) }
        pub fn meter(children: &[Fragment]) -> Fragment { tag("meter", false, children.to_vec()) }
        pub fn nav(children: &[Fragment]) -> Fragment { tag("nav", false, children.to_vec()) }
        pub fn noframes(children: &[Fragment]) -> Fragment { tag("noframes", false, children.to_vec()) }
        pub fn noscript(children: &[Fragment]) -> Fragment { tag("noscript", false, children.to_vec()) }
        pub fn object(children: &[Fragment]) -> Fragment { tag("object", false, children.to_vec()) }
        pub fn ol(children: &[Fragment]) -> Fragment { tag("ol", false, children.to_vec()) }
        pub fn optgroup(children: &[Fragment]) -> Fragment { tag("optgroup", false, children.to_vec()) }
        pub fn option(children: &[Fragment]) -> Fragment { tag("option", false, children.to_vec()) }
        pub fn output(children: &[Fragment]) -> Fragment { tag("output", false, children.to_vec()) }
        pub fn p(children: &[Fragment]) -> Fragment { tag("p", false, children.to_vec()) }
        pub fn param(children: &[Fragment]) -> Fragment { tag("param", false, children.to_vec()) }
        pub fn picture(children: &[Fragment]) -> Fragment { tag("picture", false, children.to_vec()) }
        pub fn pre(children: &[Fragment]) -> Fragment { tag("pre", false, children.to_vec()) }
        pub fn progress(children: &[Fragment]) -> Fragment { tag("progress", false, children.to_vec()) }
        pub fn q(children: &[Fragment]) -> Fragment { tag("q", false, children.to_vec()) }
        pub fn rp(children: &[Fragment]) -> Fragment { tag("rp", false, children.to_vec()) }
        pub fn rt(children: &[Fragment]) -> Fragment { tag("rt", false, children.to_vec()) }
        pub fn ruby(children: &[Fragment]) -> Fragment { tag("ruby", false, children.to_vec()) }
        pub fn s(children: &[Fragment]) -> Fragment { tag("s", false, children.to_vec()) }
        pub fn samp(children: &[Fragment]) -> Fragment { tag("samp", false, children.to_vec()) }
        pub fn script(children: &[Fragment]) -> Fragment { tag("script", false, children.to_vec()) }
        pub fn section(children: &[Fragment]) -> Fragment { tag("section", false, children.to_vec()) }
        pub fn select(children: &[Fragment]) -> Fragment { tag("select", false, children.to_vec()) }
        pub fn small(children: &[Fragment]) -> Fragment { tag("small", false, children.to_vec()) }
        pub fn source(children: &[Fragment]) -> Fragment { tag("source", false, children.to_vec()) }
        pub fn span(children: &[Fragment]) -> Fragment { tag("span", false, children.to_vec()) }
        pub fn strike(children: &[Fragment]) -> Fragment { tag("strike", false, children.to_vec()) }
        pub fn strong(children: &[Fragment]) -> Fragment { tag("strong", false, children.to_vec()) }
        pub fn style(children: &[Fragment]) -> Fragment { tag("style", false, children.to_vec()) }
        pub fn sub(children: &[Fragment]) -> Fragment { tag("sub", false, children.to_vec()) }
        pub fn summary(children: &[Fragment]) -> Fragment { tag("summary", false, children.to_vec()) }
        pub fn sup(children: &[Fragment]) -> Fragment { tag("sup", false, children.to_vec()) }
        pub fn table(children: &[Fragment]) -> Fragment { tag("table", false, children.to_vec()) }
        pub fn tbody(children: &[Fragment]) -> Fragment { tag("tbody", false, children.to_vec()) }
        pub fn td(children: &[Fragment]) -> Fragment { tag("td", false, children.to_vec()) }
        pub fn template(children: &[Fragment]) -> Fragment { tag("template", false, children.to_vec()) }
        pub fn textarea(children: &[Fragment]) -> Fragment { tag("textarea", false, children.to_vec()) }
        pub fn tfoot(children: &[Fragment]) -> Fragment { tag("tfoot", false, children.to_vec()) }
        pub fn th(children: &[Fragment]) -> Fragment { tag("th", false, children.to_vec()) }
        pub fn thead(children: &[Fragment]) -> Fragment { tag("thead", false, children.to_vec()) }
        pub fn time(children: &[Fragment]) -> Fragment { tag("time", false, children.to_vec()) }
        pub fn title(children: &[Fragment]) -> Fragment { tag("title", false, children.to_vec()) }
        pub fn tr(children: &[Fragment]) -> Fragment { tag("tr", false, children.to_vec()) }
        pub fn track(children: &[Fragment]) -> Fragment { tag("track", false, children.to_vec()) }
        pub fn tt(children: &[Fragment]) -> Fragment { tag("tt", false, children.to_vec()) }
        pub fn u(children: &[Fragment]) -> Fragment { tag("u", false, children.to_vec()) }
        pub fn ul(children: &[Fragment]) -> Fragment { tag("ul", false, children.to_vec()) }
        pub fn var(children: &[Fragment]) -> Fragment { tag("var", false, children.to_vec()) }
        pub fn video(children: &[Fragment]) -> Fragment { tag("video", false, children.to_vec()) }
        pub fn wbr(children: &[Fragment]) -> Fragment { tag("wbr", false, children.to_vec()) }
    }

    pub mod attributes {
        use rustags::core::*;

        pub fn accept(value: &str) -> Fragment { attribute("accept", value) }
        pub fn accept_charset(value: &str) -> Fragment { attribute("accept-charset", value) }
        pub fn accesskey(value: &str) -> Fragment { attribute("accesskey", value) }
        pub fn action(value: &str) -> Fragment { attribute("action", value) }
        pub fn align(value: &str) -> Fragment { attribute("align", value) }
        pub fn alt(value: &str) -> Fragment { attribute("alt", value) }
        pub fn async(value: &str) -> Fragment { attribute("async", value) }
        pub fn autocomplete(value: &str) -> Fragment { attribute("autocomplete", value) }
        pub fn autofocus(value: &str) -> Fragment { attribute("autofocus", value) }
        pub fn autoplay(value: &str) -> Fragment { attribute("autoplay", value) }
        pub fn bgcolor(value: &str) -> Fragment { attribute("bgcolor", value) }
        pub fn border(value: &str) -> Fragment { attribute("border", value) }
        pub fn charset(value: &str) -> Fragment { attribute("charset", value) }
        pub fn checked(value: &str) -> Fragment { attribute("checked", value) }
        pub fn cite(value: &str) -> Fragment { attribute("cite", value) }
        pub fn class(value: &str) -> Fragment { attribute("class", value) }
        pub fn color(value: &str) -> Fragment { attribute("color", value) }
        pub fn cols(value: &str) -> Fragment { attribute("cols", value) }
        pub fn colspan(value: &str) -> Fragment { attribute("colspan", value) }
        pub fn content(value: &str) -> Fragment { attribute("content", value) }
        pub fn contenteditable(value: &str) -> Fragment { attribute("contenteditable", value) }
        pub fn contextmenu(value: &str) -> Fragment { attribute("contextmenu", value) }
        pub fn controls(value: &str) -> Fragment { attribute("controls", value) }
        pub fn coords(value: &str) -> Fragment { attribute("coords", value) }
        pub fn data(value: &str) -> Fragment { attribute("data", value) }
        pub fn data_(value: &str) -> Fragment { attribute("data-*", value) }
        pub fn datetime(value: &str) -> Fragment { attribute("datetime", value) }
        pub fn default(value: &str) -> Fragment { attribute("default", value) }
        pub fn defer(value: &str) -> Fragment { attribute("defer", value) }
        pub fn dir(value: &str) -> Fragment { attribute("dir", value) }
        pub fn dirname(value: &str) -> Fragment { attribute("dirname", value) }
        pub fn disabled(value: &str) -> Fragment { attribute("disabled", value) }
        pub fn download(value: &str) -> Fragment { attribute("download", value) }
        pub fn draggable(value: &str) -> Fragment { attribute("draggable", value) }
        pub fn dropzone(value: &str) -> Fragment { attribute("dropzone", value) }
        pub fn enctype(value: &str) -> Fragment { attribute("enctype", value) }
        pub fn for_(value: &str) -> Fragment { attribute("for", value) }
        pub fn form(value: &str) -> Fragment { attribute("form", value) }
        pub fn formaction(value: &str) -> Fragment { attribute("formaction", value) }
        pub fn headers(value: &str) -> Fragment { attribute("headers", value) }
        pub fn height(value: &str) -> Fragment { attribute("height", value) }
        pub fn hidden(value: &str) -> Fragment { attribute("hidden", value) }
        pub fn high(value: &str) -> Fragment { attribute("high", value) }
        pub fn href(value: &str) -> Fragment { attribute("href", value) }
        pub fn hreflang(value: &str) -> Fragment { attribute("hreflang", value) }
        pub fn http_equiv(value: &str) -> Fragment { attribute("http-equiv", value) }
        pub fn id(value: &str) -> Fragment { attribute("id", value) }
        pub fn ismap(value: &str) -> Fragment { attribute("ismap", value) }
        pub fn kind(value: &str) -> Fragment { attribute("kind", value) }
        pub fn label(value: &str) -> Fragment { attribute("label", value) }
        pub fn lang(value: &str) -> Fragment { attribute("lang", value) }
        pub fn list(value: &str) -> Fragment { attribute("list", value) }
        pub fn _loop(value: &str) -> Fragment { attribute("loop", value) }
        pub fn low(value: &str) -> Fragment { attribute("low", value) }
        pub fn max(value: &str) -> Fragment { attribute("max", value) }
        pub fn maxlength(value: &str) -> Fragment { attribute("maxlength", value) }
        pub fn media(value: &str) -> Fragment { attribute("media", value) }
        pub fn method(value: &str) -> Fragment { attribute("method", value) }
        pub fn min(value: &str) -> Fragment { attribute("min", value) }
        pub fn multiple(value: &str) -> Fragment { attribute("multiple", value) }
        pub fn muted(value: &str) -> Fragment { attribute("muted", value) }
        pub fn name(value: &str) -> Fragment { attribute("name", value) }
        pub fn novalidate(value: &str) -> Fragment { attribute("novalidate", value) }
        pub fn onabort(value: &str) -> Fragment { attribute("onabort", value) }
        pub fn onafterprint(value: &str) -> Fragment { attribute("onafterprint", value) }
        pub fn onbeforeprint(value: &str) -> Fragment { attribute("onbeforeprint", value) }
        pub fn onbeforeunload(value: &str) -> Fragment { attribute("onbeforeunload", value) }
        pub fn onblur(value: &str) -> Fragment { attribute("onblur", value) }
        pub fn oncanplay(value: &str) -> Fragment { attribute("oncanplay", value) }
        pub fn oncanplaythrough(value: &str) -> Fragment { attribute("oncanplaythrough", value) }
        pub fn onchange(value: &str) -> Fragment { attribute("onchange", value) }
        pub fn onclick(value: &str) -> Fragment { attribute("onclick", value) }
        pub fn oncontextmenu(value: &str) -> Fragment { attribute("oncontextmenu", value) }
        pub fn oncopy(value: &str) -> Fragment { attribute("oncopy", value) }
        pub fn oncuechange(value: &str) -> Fragment { attribute("oncuechange", value) }
        pub fn oncut(value: &str) -> Fragment { attribute("oncut", value) }
        pub fn ondblclick(value: &str) -> Fragment { attribute("ondblclick", value) }
        pub fn ondrag(value: &str) -> Fragment { attribute("ondrag", value) }
        pub fn ondragend(value: &str) -> Fragment { attribute("ondragend", value) }
        pub fn ondragenter(value: &str) -> Fragment { attribute("ondragenter", value) }
        pub fn ondragleave(value: &str) -> Fragment { attribute("ondragleave", value) }
        pub fn ondragover(value: &str) -> Fragment { attribute("ondragover", value) }
        pub fn ondragstart(value: &str) -> Fragment { attribute("ondragstart", value) }
        pub fn ondrop(value: &str) -> Fragment { attribute("ondrop", value) }
        pub fn ondurationchange(value: &str) -> Fragment { attribute("ondurationchange", value) }
        pub fn onemptied(value: &str) -> Fragment { attribute("onemptied", value) }
        pub fn onended(value: &str) -> Fragment { attribute("onended", value) }
        pub fn onerror(value: &str) -> Fragment { attribute("onerror", value) }
        pub fn onfocus(value: &str) -> Fragment { attribute("onfocus", value) }
        pub fn onhashchange(value: &str) -> Fragment { attribute("onhashchange", value) }
        pub fn oninput(value: &str) -> Fragment { attribute("oninput", value) }
        pub fn oninvalid(value: &str) -> Fragment { attribute("oninvalid", value) }
        pub fn onkeydown(value: &str) -> Fragment { attribute("onkeydown", value) }
        pub fn onkeypress(value: &str) -> Fragment { attribute("onkeypress", value) }
        pub fn onkeyup(value: &str) -> Fragment { attribute("onkeyup", value) }
        pub fn onload(value: &str) -> Fragment { attribute("onload", value) }
        pub fn onloadeddata(value: &str) -> Fragment { attribute("onloadeddata", value) }
        pub fn onloadedmetadata(value: &str) -> Fragment { attribute("onloadedmetadata", value) }
        pub fn onloadstart(value: &str) -> Fragment { attribute("onloadstart", value) }
        pub fn onmousedown(value: &str) -> Fragment { attribute("onmousedown", value) }
        pub fn onmousemove(value: &str) -> Fragment { attribute("onmousemove", value) }
        pub fn onmouseout(value: &str) -> Fragment { attribute("onmouseout", value) }
        pub fn onmouseover(value: &str) -> Fragment { attribute("onmouseover", value) }
        pub fn onmouseup(value: &str) -> Fragment { attribute("onmouseup", value) }
        pub fn onmousewheel(value: &str) -> Fragment { attribute("onmousewheel", value) }
        pub fn onoffline(value: &str) -> Fragment { attribute("onoffline", value) }
        pub fn ononline(value: &str) -> Fragment { attribute("ononline", value) }
        pub fn onpagehide(value: &str) -> Fragment { attribute("onpagehide", value) }
        pub fn onpageshow(value: &str) -> Fragment { attribute("onpageshow", value) }
        pub fn onpaste(value: &str) -> Fragment { attribute("onpaste", value) }
        pub fn onpause(value: &str) -> Fragment { attribute("onpause", value) }
        pub fn onplay(value: &str) -> Fragment { attribute("onplay", value) }
        pub fn onplaying(value: &str) -> Fragment { attribute("onplaying", value) }
        pub fn onpopstate(value: &str) -> Fragment { attribute("onpopstate", value) }
        pub fn onprogress(value: &str) -> Fragment { attribute("onprogress", value) }
        pub fn onratechange(value: &str) -> Fragment { attribute("onratechange", value) }
        pub fn onreset(value: &str) -> Fragment { attribute("onreset", value) }
        pub fn onresize(value: &str) -> Fragment { attribute("onresize", value) }
        pub fn onscroll(value: &str) -> Fragment { attribute("onscroll", value) }
        pub fn onsearch(value: &str) -> Fragment { attribute("onsearch", value) }
        pub fn onseeked(value: &str) -> Fragment { attribute("onseeked", value) }
        pub fn onseeking(value: &str) -> Fragment { attribute("onseeking", value) }
        pub fn onselect(value: &str) -> Fragment { attribute("onselect", value) }
        pub fn onshow(value: &str) -> Fragment { attribute("onshow", value) }
        pub fn onstalled(value: &str) -> Fragment { attribute("onstalled", value) }
        pub fn onstorage(value: &str) -> Fragment { attribute("onstorage", value) }
        pub fn onsubmit(value: &str) -> Fragment { attribute("onsubmit", value) }
        pub fn onsuspend(value: &str) -> Fragment { attribute("onsuspend", value) }
        pub fn ontimeupdate(value: &str) -> Fragment { attribute("ontimeupdate", value) }
        pub fn ontoggle(value: &str) -> Fragment { attribute("ontoggle", value) }
        pub fn onunload(value: &str) -> Fragment { attribute("onunload", value) }
        pub fn onvolumechange(value: &str) -> Fragment { attribute("onvolumechange", value) }
        pub fn onwaiting(value: &str) -> Fragment { attribute("onwaiting", value) }
        pub fn onwheel(value: &str) -> Fragment { attribute("onwheel", value) }
        pub fn open(value: &str) -> Fragment { attribute("open", value) }
        pub fn optimum(value: &str) -> Fragment { attribute("optimum", value) }
        pub fn pattern(value: &str) -> Fragment { attribute("pattern", value) }
        pub fn placeholder(value: &str) -> Fragment { attribute("placeholder", value) }
        pub fn poster(value: &str) -> Fragment { attribute("poster", value) }
        pub fn preload(value: &str) -> Fragment { attribute("preload", value) }
        pub fn readonly(value: &str) -> Fragment { attribute("readonly", value) }
        pub fn rel(value: &str) -> Fragment { attribute("rel", value) }
        pub fn required(value: &str) -> Fragment { attribute("required", value) }
        pub fn reversed(value: &str) -> Fragment { attribute("reversed", value) }
        pub fn rows(value: &str) -> Fragment { attribute("rows", value) }
        pub fn rowspan(value: &str) -> Fragment { attribute("rowspan", value) }
        pub fn sandbox(value: &str) -> Fragment { attribute("sandbox", value) }
        pub fn scope(value: &str) -> Fragment { attribute("scope", value) }
        pub fn scoped(value: &str) -> Fragment { attribute("scoped", value) }
        pub fn selected(value: &str) -> Fragment { attribute("selected", value) }
        pub fn shape(value: &str) -> Fragment { attribute("shape", value) }
        pub fn size(value: &str) -> Fragment { attribute("size", value) }
        pub fn sizes(value: &str) -> Fragment { attribute("sizes", value) }
        pub fn span(value: &str) -> Fragment { attribute("span", value) }
        pub fn spellcheck(value: &str) -> Fragment { attribute("spellcheck", value) }
        pub fn src(value: &str) -> Fragment { attribute("src", value) }
        pub fn srcdoc(value: &str) -> Fragment { attribute("srcdoc", value) }
        pub fn srclang(value: &str) -> Fragment { attribute("srclang", value) }
        pub fn srcset(value: &str) -> Fragment { attribute("srcset", value) }
        pub fn start(value: &str) -> Fragment { attribute("start", value) }
        pub fn step(value: &str) -> Fragment { attribute("step", value) }
        pub fn style(value: &str) -> Fragment { attribute("style", value) }
        pub fn tabindex(value: &str) -> Fragment { attribute("tabindex", value) }
        pub fn target(value: &str) -> Fragment { attribute("target", value) }
        pub fn title(value: &str) -> Fragment { attribute("title", value) }
        pub fn translate(value: &str) -> Fragment { attribute("translate", value) }
        pub fn _type(value: &str) -> Fragment { attribute("type", value) }
        pub fn usemap(value: &str) -> Fragment { attribute("usemap", value) }
        pub fn value(value: &str) -> Fragment { attribute("value", value) }
        pub fn width(value: &str) -> Fragment { attribute("width", value) }
        pub fn wrap(value: &str) -> Fragment { attribute("wrap", value) }
    }
}
