import init, {
  // initThreadPool,
  init_client,
} from "../../../static/pkg/mxyz_client.js";

let script = document.getElementById("simulation.js");

let category = script.getAttribute("category");
let simulation_variant = script.getAttribute("simulation_variant");

(async () => {
  await init();
  // await initThreadPool(navigator.hardwareConcurrency);
  init_client(2, category, simulation_variant);
})();
