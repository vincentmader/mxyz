console.log("Hello, client!");

// // import init, { SimulationClient } from "../../../static/pkg/mxyz_client.js";
// import init, { MXYZClient } from "../../pkg/mxyz_client.js";

// let sim_id = document.getElementById("simulation.js").getAttribute("sim_id");

// (async () => {
//   await init();

//   // initialization
//   let simulation = MXYZClient.new();
//   simulation.init();
//   simulation.start();

//   //   // TODO move this definition to rust?
//   //   let NR_OF_ITERATIONS_PER_RENDER = 5;
//   //   let bm = document.getElementById("button-menu_main");
//   //   let slider = document.createElement("input");
//   //   slider.type = "range";
//   //   slider.id = "slider_set-iterations-per-render";
//   //   slider.min = 0;
//   //   slider.max = 100;
//   //   slider.value = 1; // if changed: also in sim/mod.rs AND/OR sim/config.rs
//   //   bm.appendChild(slider);
//   //   slider.style = "width: 100%";

//   //   // event listeners for options
//   //   var options = document.getElementsByTagName("select");
//   //   for (let idx = 0; idx < options.length; idx++) {
//   //     let option = options[idx];
//   //     option.addEventListener("change", () => {
//   //       simulation.handle_option_event(option.id);
//   //     });
//   //   }

//   //   // event listeners for buttons
//   //   var buttons = document.getElementsByTagName("button");
//   //   for (let idx = 0; idx < buttons.length; idx++) {
//   //     let button = buttons[idx];
//   //     button.addEventListener("click", () => {
//   //       simulation.handle_button_event(button.id);
//   //     });
//   //   }

//   //   // event listeners for sliders
//   //   var inputs = document.getElementsByTagName("input");
//   //   for (let idx = 0; idx < inputs.length; idx++) {
//   //     let slider = inputs[idx];
//   //     if (slider.getAttribute("type") != "range") continue;
//   //     slider.addEventListener("change", () => {
//   //       simulation.handle_slider_event(slider.id);
//   //     });
//   //   }

//   //   // let last_date = Date.now();
//   //   const loop = () => {
//   //     let date_1 = Date.now();
//   //     simulation.render();
//   //     let date_2 = Date.now();
//   //     let fps_renderer = 1000 / (date_2 - date_1);

//   //     let date_3 = Date.now();
//   //     simulation.step(); // TODO move render into sim.step() ?
//   //     let date_4 = Date.now();
//   //     let fps_engine = 1000 / (date_4 - date_3);

//   //     let textfield_fps_e = document.getElementById("textfield_fps_engine");
//   //     textfield_fps_e.innerHTML = "fps_e: " + Math.round(fps_engine);
//   //     let textfield_fps_r = document.getElementById("textfield_fps_renderer");
//   //     textfield_fps_r.innerHTML = "fps_r: " + Math.round(fps_renderer);

//   //     // let dt = (date_2 - date_1) / 1000;
//   //     // let fps = 1 / dt;
//   //     // console.log(fps);

//   //     // setTimeout(function () {
//   //     requestAnimationFrame(loop);
//   //     // }, 500);
//   //   };
//   //   loop();
// })();
