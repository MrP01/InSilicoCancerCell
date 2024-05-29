import { run } from "./pkg/in_silico_frontend";
import * as Plot from "@observablehq/plot";
import patchclampdata from "./pkg/patchclampdata-g0-activation-sub225.json";

const simulation = run();

function fullSimulationCurrent({}, interactive = false) {
  const sharedX = [...Array(simulation.total_current.length).keys()].map(
    (x) => x * (9.901 / simulation.total_current.length)
  );
  return {
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Current / pA" }),
      // Plot.lineY(simulation.voltage, {
      //   y: (y) => y * 1e3,
      //   z: null,
      //   tip: interactive ? "x" : undefined,
      // }),
      Plot.lineY(simulation.total_current, {
        x: sharedX,
        y: (y) => y,
        z: null,
        stroke: (y) => y,
        tip: interactive ? "x" : undefined,
      }),
      // @ts-ignore
      Plot.lineY(patchclampdata.current[0], {
        x: sharedX,
        y: (y) => y,
        z: null,
        tip: interactive ? "x" : undefined,
      }),
    ],
  };
}

function channelCurrent({ channel }, interactive = false) {
  const current = simulation.channels.get(channel).current;
  return {
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Current / pA" }),
      Plot.lineY(current, {
        x: [...Array(current.length).keys()].map((x) => x * (9.901 / current.length)),
        y: (y) => y,
        z: null,
        stroke: (y) => y,
        tip: interactive ? "x" : undefined,
      }),
    ],
    width: 600,
    height: 240,
  };
}

function channelState({ channel }, interactive = false) {
  let record = simulation.channels.get(channel).states;
  let n_states = record[0].length;
  console.log("channel", channel, interactive, "has", n_states);
  let tidy = [];
  for (let step = 0; step < record.length; step++) {
    for (let state = 0; state < n_states; state++) {
      tidy.push({ step, state: state, value: record[step][state] });
    }
  }
  return {
    color: {
      scheme: "observable10",
    },
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "State Composition" }),
      Plot.areaY(tidy, {
        x: (e) => e.step * (9.901 / 800),
        y: "value",
        fill: "state",
        reverse: true,
        tip: interactive ? "xy" : undefined,
      }),
    ],
    width: 600,
    height: 240,
  };
}

const ALL_PLOT_GENERATORS = { fullSimulationCurrent, channelCurrent, channelState };
export function generatePlot(name, args = {}, interactive = false) {
  console.log(name, "args", args);
  return ALL_PLOT_GENERATORS[name](args, (interactive = interactive));
}
