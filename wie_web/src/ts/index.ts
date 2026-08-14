import { WieWeb } from "@pkg";
import { setMasterVolume } from "./midi";

const key_map: Record<string, string> = {
  Digit1: "1", Digit2: "2", Digit3: "3",
  KeyQ: "4", KeyW: "5", KeyE: "6",
  KeyA: "7", KeyS: "8", KeyD: "9",
  KeyZ: "*", KeyX: "0", KeyC: "#",
  Backspace: "CLR", ArrowUp: "UP", ArrowLeft: "LEFT",
  ArrowRight: "RIGHT", ArrowDown: "DOWN", Space: "OK",
};

let emulator: WieWeb | null = null;
let animationStarted = false;

const setStatus = (message: string) => {
  const el = document.getElementById("status");
  if (el) el.textContent = message;
};

const installDebugConsoleMirror = () => {
  const scm2Logs: string[] = [];

  const addScm2Log = (message: string) => {
    scm2Logs.push(message);
    if (scm2Logs.length > 30) scm2Logs.shift();
    setStatus(scm2Logs.join("\n"));
  };

  for (const level of ["log", "info", "warn", "error"] as const) {
    const original = console[level].bind(console);
    console[level] = (...args: unknown[]) => {
      original(...args);
      const message = args.map(String).join(" ");

      if (message.includes("SCM2 URL.find:")) {
        const pos = message.indexOf("SCM2 URL.find:");
        addScm2Log(message.slice(pos));
      } else if (message.includes("SCM2 SOCKET WRITE_ARRAY:")) {
        const pos = message.indexOf("SCM2 SOCKET WRITE_ARRAY:");
        addScm2Log(message.slice(pos));
      } else if (message.includes("SCM2 SOCKET WRITE:")) {
        const pos = message.indexOf("SCM2 SOCKET WRITE:");
        addScm2Log(message.slice(pos));
      } else if (message.includes("SCM2 SOCKET:")) {
        const pos = message.indexOf("SCM2 SOCKET:");
        addScm2Log(message.slice(pos));
      } else if (message.includes("SCM2 JAVA SVC:")) {
        const pos = message.indexOf("SCM2 JAVA SVC:");
        addScm2Log(message.slice(pos));
      } else if (message.includes("SCM2 JAVA_BODY:")) {
        const pos = message.indexOf("SCM2 JAVA_BODY:");
        addScm2Log(message.slice(pos));
      } else if (
        message.includes("SCM2 GET_METHOD:") &&
        !message.includes("CALLSITE REG") &&
        !message.includes("CALLSITE STACK")
      ) {
        const pos = message.indexOf("SCM2 GET_METHOD:");
        addScm2Log(message.slice(pos));
      }
    };
  }
};

const bindControls = (wie_web: WieWeb) => {
  for (const button of document.querySelectorAll("button[data-key]")) {
    const down = (e: Event) => {
      e.preventDefault();
      const key = (e.currentTarget as HTMLButtonElement).dataset.key;
      if (key) wie_web.key_down(key);
    };
    const up = (e: Event) => {
      e.preventDefault();
      const key = (e.currentTarget as HTMLButtonElement).dataset.key;
      if (key) wie_web.key_up(key);
    };
    button.addEventListener("pointerdown", down);
    button.addEventListener("pointerup", up);
    button.addEventListener("pointercancel", up);
    button.addEventListener("pointerleave", up);
  }

  document.addEventListener("keydown", (e) => {
    const key = key_map[e.code];
    if (key) { e.preventDefault(); wie_web.key_down(key); }
  });
  document.addEventListener("keyup", (e) => {
    const key = key_map[e.code];
    if (key) { e.preventDefault(); wie_web.key_up(key); }
  });
};

const startUpdateLoop = (wie_web: WieWeb) => {
  if (animationStarted) return;
  animationStarted = true;
  const update = () => {
    try {
      wie_web.update();
      requestAnimationFrame(update);
    } catch (e) {
      const detail = e instanceof Error
        ? `${e.name}: ${e.message}\n${e.stack ?? "(stack 없음)"}`
        : String(e);
      setStatus(`실행 오류:\n${detail}`);
      console.error(e);
    }
  };
  requestAnimationFrame(update);
};

const initSettings = () => {
  const toggle = document.getElementById("settings-toggle");
  const panel = document.getElementById("settings-panel");
  const midiSlider = document.getElementById("volume-midi") as HTMLInputElement;
  const pcmSlider = document.getElementById("volume-pcm") as HTMLInputElement;

  setMasterVolume(Number(midiSlider.value) / 100);
  midiSlider.addEventListener("input", () => setMasterVolume(Number(midiSlider.value) / 100));
  pcmSlider.addEventListener("input", () => emulator?.set_pcm_volume(Number(pcmSlider.value) / 100));
  toggle?.addEventListener("click", () => panel?.classList.toggle("visible"));
};

const bootBundledGame = async () => {
  setStatus("소울카드마스터2 불러오는 중…");
  const response = await fetch("scm2.wipi.zip");
  if (!response.ok) throw new Error(`내장 게임 파일을 읽을 수 없습니다 (${response.status})`);
  const data = new Uint8Array(await response.arrayBuffer());
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;

  emulator = new WieWeb("scm2.wipi.zip", data, canvas);
  const pcmSlider = document.getElementById("volume-pcm") as HTMLInputElement;
  emulator.set_pcm_volume(Number(pcmSlider.value) / 100);
  bindControls(emulator);
  startUpdateLoop(emulator);
  setStatus("실행 중");
};

const main = async () => {
  installDebugConsoleMirror();
  initSettings();
  try {
    await bootBundledGame();
  } catch (e) {
    setStatus(`시작 실패: ${String(e)}`);
    console.error(e);
  }
};

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", main, { once: true });
} else {
  void main();
}
