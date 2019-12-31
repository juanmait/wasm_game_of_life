import { run } from "./run";

export const init = async () => {
  const init = await import("../pkg/index.js");
  const { memory } = await import("../pkg/index_bg");

  run(init, memory).catch(err => console.error(err));
};

init().catch(err => console.error(err));
