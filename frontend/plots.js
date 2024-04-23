import { run } from "./pkg/in_silico_frontend";
import * as Plot from "@observablehq/plot";

function fullSimulationCurrent(interactive = false) {
  return {
    marks: [Plot.lineY(run(), { y: (y) => y * 1e12, z: null, stroke: (y) => y, tip: interactive ? "x" : undefined })],
  };
}

const ALL_PLOT_GENERATORS = { fullSimulationCurrent };
export function generatePlot(name, interactive = false) {
  return ALL_PLOT_GENERATORS[name]((interactive = interactive));
}
