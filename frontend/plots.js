import { run } from "./pkg/in_silico_frontend";
import * as Plot from "@observablehq/plot";

const simulation = run();

function fullSimulationCurrent(interactive = false) {
  return {
    marks: [
      Plot.lineY(simulation.total_current, {
        y: (y) => y * 1e12,
        z: null,
        stroke: (y) => y,
        tip: interactive ? "x" : undefined,
      }),
    ],
  };
}

function channelCurrent(interactive = false) {
  return {};
}
function channelState(interactive = false) {
  return {};
}

const ALL_PLOT_GENERATORS = { fullSimulationCurrent, channelCurrent, channelState };
export function generatePlot(name, args = [], interactive = false) {
  return ALL_PLOT_GENERATORS[name](...args, (interactive = interactive));
}
