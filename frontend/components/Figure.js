import * as Plot from "@observablehq/plot";
import Document from "./Document";

export default function Figure({ options }) {
  return Plot.plot({ ...options, document: new Document() }).toHyperScript();
}
