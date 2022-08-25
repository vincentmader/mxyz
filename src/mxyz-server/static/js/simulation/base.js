import init, {
  // initThreadPool, (?)
  SimulationClientV1,
} from "../../../static/pkg/mxyz_client.js";

let script = document.getElementById("simulation.js");
let category = script.getAttribute("category");
let simulation_variant = script.getAttribute("simulation_variant");

console.log(
  "category-id: \t\t\t" +
    category +
    "\nsimulation-variant: \t" +
    simulation_variant
);

(async () => {
  await init();
  let simulation = SimulationClientV1.new();
  simulation.init(simulation_variant);
})();
