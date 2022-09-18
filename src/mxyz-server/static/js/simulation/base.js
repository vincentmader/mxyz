import init, {
  init_client, // initThreadPool,
} from "../../../static/pkg/mxyz_client.js";

let script = document.getElementById("simulation.js");
let category = script.getAttribute("category");
let simulation_variant = script.getAttribute("simulation_variant");

(async () => {
  await init(); // await initThreadPool(navigator.hardwareConcurrency);
  init_client(1, category, simulation_variant);
})();
