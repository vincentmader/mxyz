import init, {
  // initThreadPool,
  SimulationClientV2Render,
  SimulationClientV1Compute,
} from "../../../static/pkg/mxyz_client.js";

let script = document.getElementById("simulation.js");

let category = script.getAttribute("category");
let simulation_variant = script.getAttribute("simulation_variant");

(async () => {
  await init();
  // await initThreadPool(navigator.hardwareConcurrency);

  let simulation = SimulationClientV2Render.new();
  simulation.init(category, simulation_variant);
})();
