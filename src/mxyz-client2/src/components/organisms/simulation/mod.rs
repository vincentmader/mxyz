use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../../../mxyz-server/static/css/simulation/base.css");

#[derive(PartialEq, yew::Properties, std::default::Default, Clone)]
pub struct Props {}

#[function_component(Simulation)]
pub fn fn_name(_props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    html! {
        <div class="page_content">
            <div class={style}>
                <div id="page-columns">
                    <div id="page-column-left">
                        <div class="canvas_container">
                            <canvas id="canvas_0" width="1000px" height="1000px"> </canvas>
                            // <script>
                            //     let cnv = document.getElementById("canvas_0");
                            //     cnv.height = cnv.width;  // TODO make changeable
                            // </script>
                        </div>
                    </div>
                    <div id="page-column-right"> </div>
                </div>
            </div>
        </div>
    }
}
