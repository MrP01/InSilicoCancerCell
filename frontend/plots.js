import { run, get_protocol_sample } from "./pkg/in_silico_frontend";
import * as Plot from "@observablehq/plot";
import { selector } from "./store";
import { movingAverage } from "./utils";

var simulation;
selector.subscribe((value) => {
  simulation = run(value.protocol, value.phase);
  console.log("Simulation done", value);
});

async function getMeasurements() {
  const { protocol, phase } = selector.get();
  let i = await import(`./pkg/patchclampdata-${phase}-${protocol}.json`);
  // console.log(i);
  return i;
}

async function fullSimulationCurrent({}, interactive = false) {
  const sharedX = [...Array(simulation.total_current.length).keys()].map(
    (x) => x * (9.901 / simulation.total_current.length)
  );
  const measurements = await getMeasurements();
  return {
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Current / pA" }),
      // Plot.lineY(simulation.voltage, {
      //   x: sharedX,
      //   y: (y) => y * 1e3,
      //   z: null,
      //   stroke: "gray",
      //   tip: interactive ? "x" : undefined,
      // }),
      Plot.lineY(simulation.total_current, {
        x: sharedX,
        y: (y) => y,
        z: null,
        stroke: (y) => y,
        strokeWidth: 4,
        // tip: interactive ? "x" : undefined,
      }),
      // @ts-ignore
      Plot.lineY(measurements.current[0], {
        x: sharedX,
        y: (y) => y,
        z: null,
        tip: interactive ? "x" : undefined,
      }),
    ],
  };
}

async function dtScale({}, interactive = false) {
  const sharedX = [...Array(simulation.total_current.length).keys()].map(
    (x) => x * (9.901 / simulation.total_current.length)
  );
  return {
    color: {
      scheme: "turbo",
    },
    x: { ticks: [] },
    marks: [Plot.ruleX(simulation.dt, { x: sharedX, stroke: (y) => y, strokeWidth: 4 })],
  };
}

async function dtScalePlot({}, interactive = false) {
  const sharedX = [...Array(simulation.total_current.length).keys()].map(
    (x) => x * (9.901 / simulation.total_current.length)
  );
  return {
    color: {
      scheme: "turbo",
    },
    y: { type: "log" },
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Time Step dt / µs" }),
      Plot.lineY(movingAverage(simulation.dt, 20), {
        x: sharedX,
        y: (y) => y * 1e6,
        z: null,
        stroke: (y) => -Math.log10(y),
      }),
    ],
    // marks: [
    //   Plot.ruleX(
    //     simulation.dt,
    //     // Plot.windowX(5, { y: simulation.dt }),
    //     // simulation.dt.map((x) => 1 / x),
    //     { x: sharedX, stroke: (y) => y, strokeWidth: 4 }
    //   ),
    // ],
  };
}

async function simulationError({}, interactive = false) {
  const sharedX = [...Array(simulation.total_current.length).keys()].map(
    (x) => x * (9.901 / simulation.total_current.length)
  );
  const measurements = await getMeasurements();
  const minLength = Math.min(simulation.total_current.length, measurements.current[0].length) * 0.95;
  let diff = [];
  for (let i = 0; i < minLength; i++) {
    const delta = simulation.total_current[i] - measurements.current[0][i];
    diff.push(Math.abs(delta));
  }
  return {
    // y: { type: "log" },
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Absolute Error / pA" }),
      // @ts-ignore
      Plot.lineY(diff, {
        x: sharedX,
        y: (y) => y,
        z: null,
        stroke: (y) => Math.pow(y, 0.4),
        // tip: interactive ? "x" : undefined,
      }),
    ],
  };
}

async function channelCurrent({ channel }, interactive = false) {
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
        strokeWidth: 3,
        tip: interactive ? "x" : undefined,
      }),
    ],
    width: 600,
    height: 240,
  };
}

async function channelState({ channel }, interactive = false) {
  let record = simulation.channels.get(channel).states;
  let n_states = record[0].length;
  // console.log("channel", channel, interactive, "has", n_states);
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

async function protocol({ protocol, cut = true }) {
  const sample = get_protocol_sample(protocol);
  const voltage = cut ? sample.slice(0, (800 / 9) * 3) : sample;
  return {
    marks: [
      Plot.axisX({ label: "Time / s" }),
      Plot.axisY({ label: "Voltage / V" }),
      Plot.lineY(voltage, {
        y: (y) => y * 1e3,
        z: null,
        stroke: "#358bf8",
      }),
    ],
    width: 150,
    height: 120,
  };
}

async function deltaTolerancePlot({}, interactive = false) {
  let data = (await import("./pkg/delta-tolerance.json")).default;
  // let base = { x: "tolerance", tip: interactive ? "x" : undefined, basis: "extent" };
  return {
    x: { type: "log", reverse: true },
    color: { legend: true },
    marks: [
      Plot.axisX({ label: "Delta Tolerance / 1", tickFormat: "e" }),
      Plot.axisY({ label: "Normalized Quantities / 1" }),
      Plot.lineY(
        data,
        Plot.normalizeY(
          Plot.windowY(4, {
            y: (e) => e.average_dt,
            stroke: "blue",
            x: "tolerance",
            tip: interactive ? "x" : undefined,
            basis: "extent",
          })
        )
      ),
      Plot.lineY(
        data,
        Plot.normalizeY(
          Plot.windowY(4, {
            y: (e) => e.accept_rate,
            stroke: "green",
            x: "tolerance",
            tip: interactive ? "x" : undefined,
            basis: "extent",
          })
        )
      ),
      Plot.lineY(
        data,
        Plot.normalizeY(
          Plot.windowY(4, {
            y: (e) => e.runtime,
            stroke: "violet",
            x: "tolerance",
            tip: interactive ? "x" : undefined,
            basis: "extent",
          })
        )
      ),
      // Plot.lineY(data, Plot.normalizeY({ y: (e) => e.iterations_k, stroke: "orange", x: "tolerance", tip: interactive ? "x" : undefined, basis: "extent"})),
      // Plot.lineY(data, Plot.normalizeY({ y: (e) => e.error, stroke: "red", x: "tolerance", tip: interactive ? "x" : undefined, basis: "extent"})),
      Plot.dot([{ label: "runtime", stroke: "violet" }]),
    ],
  };
}

const ALL_PLOT_GENERATORS = {
  fullSimulationCurrent,
  channelCurrent,
  channelState,
  protocol,
  simulationError,
  dtScale,
  dtScalePlot,
  deltaTolerancePlot,
};
export async function generatePlot(name, args = {}, interactive = false) {
  // console.log(name, "args", args);
  return await ALL_PLOT_GENERATORS[name](args, (interactive = interactive));
}
