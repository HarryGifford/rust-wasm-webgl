export type UserEvents = {
  /** Called continuously as the mouse moves over the canvas. */
  mousemove?: (screenX: number, screenY: number, clicked: boolean) => void;
  /** Called when the canvas is resized. */
  resize?: (width: number, height: number) => void;
};

/** Pressed buttons act as bit vector flags. */
const enum ButtonFlags {
  Primary = 1,
}

const primaryButtonPressed = ({ buttons }: MouseEvent) =>
  (buttons & ButtonFlags.Primary) === ButtonFlags.Primary;

/** Bind a set of event handlers to the given canvas. */
export const bindHandlers = (
  canvas: HTMLCanvasElement,
  { mousemove, resize }: UserEvents
) => {
  if (mousemove != null) {
    canvas.addEventListener(
      "mousemove",
      (e) => {
        return mousemove(e.screenX, e.screenY, primaryButtonPressed(e));
      },
      {
        passive: true,
      }
    );
  }

  const parentEl = canvas.parentElement;
  if (resize != null && parentEl != null) {
    window.addEventListener("resize", e => {
      const width = Math.floor(parentEl.clientWidth);
      const height = Math.floor(parentEl.clientHeight);
      canvas.width = width;
      canvas.height = height;
      return resize(width, height);
    });
  }
};
