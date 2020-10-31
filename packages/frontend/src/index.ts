import { Context } from "demo-rust";
import { createCanvas, createGlContext } from "./canvas-utils";
import { loadText } from "./load-text";
import { bindHandlers } from "./user-input";

const main = async () => {
  const el = document.body;
  const canvas = createCanvas(el);
  const gl = createGlContext(canvas);
  const ctx = new Context(canvas.width, canvas.height);

  const [vertexShaderSrc, fragShaderSrc] = await Promise.all([
    loadText("main.vert"),
    loadText("main.frag"),
  ]);
  ctx.init_shaders(gl, vertexShaderSrc, fragShaderSrc);
  bindHandlers(canvas, {
    mousemove: ctx.mousemove.bind(ctx),
    resize: ctx.resize.bind(ctx),
  });
  ctx.render(gl, canvas.width, canvas.height);
  (window as any).gl = gl;
};

main().catch(console.error);
