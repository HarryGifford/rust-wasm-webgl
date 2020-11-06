import { Context } from "demo-rust";
import { createCanvas, createGlContext } from "./canvas-utils";
import { loadText } from "./load-text";
import { bindHandlers } from "./user-input";

const main = async () => {
  const el = document.body;
  const canvas = createCanvas(el);
  const gl = createGlContext(canvas);
  const ctx = new Context(canvas.width, canvas.height);
  console.log("context", ctx);

  const [vertexShaderSrc, fragShaderSrc] = await Promise.all([
    loadText("main.vert"),
    loadText("main.frag"),
  ]);
  ctx.init(gl, vertexShaderSrc, fragShaderSrc);
  bindHandlers(canvas, {
    mousemove: (x, y, clicked) => {
      ctx.mousemove(gl, x, y, clicked);
      ctx.render(gl);
    },
    resize: (width, height) => {
      ctx.resize(gl, width, height);
    },
  });
  (window as any).gl = gl;
  ctx.render(gl);
};

main().catch(console.error);
