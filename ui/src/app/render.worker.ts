/// <reference lib="webworker" />

import init, { render_rows } from "raytracer";

let initialized = false;

export interface SceneData {
  width: number;
  height: number;
  startRow: number;
  endRow: number;
  sphereData: Float32Array;
  triangleData: Float32Array;
  lightData: Float32Array;
  diffuseIntensity: number;
  skyboxPixels: Float32Array;
  skyboxWidth: number;
  skyboxHeight: number;
  skyboxBrightness: number;
  cloudData: Float32Array;
  samplesPerAxis: number;
};

addEventListener("message", async ({ data }: MessageEvent<SceneData>) => {
  const { width, height, startRow, endRow, sphereData, triangleData, lightData, diffuseIntensity, skyboxPixels, skyboxWidth, skyboxHeight, skyboxBrightness, cloudData, samplesPerAxis } = data;

  if (!initialized) {
    await init({ module_or_path: "./raytracer_bg.wasm" });
    initialized = true;
  }

  const pixels = render_rows(width, height, startRow, endRow, sphereData, triangleData, lightData, diffuseIntensity, skyboxPixels, skyboxWidth, skyboxHeight, skyboxBrightness, cloudData, samplesPerAxis);
  postMessage({ startRow, endRow, pixels }, [pixels.buffer] as any);
});
