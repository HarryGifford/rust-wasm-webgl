import { render, mousemove, resize } from "demo-rust";
import { createCanvas, createGlContext } from "./canvas-utils";
import { bindHandlers } from "./user-input";

const main = () => {
  const el = document.body;
  const canvas = createCanvas(el);
  const gl = createGlContext(canvas);
  bindHandlers(canvas, {
    mousemove,
    resize
  })
  render(gl, canvas.width, canvas.height);
}

main();
