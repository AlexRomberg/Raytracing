import { Component, effect, ElementRef, inject, signal, untracked, viewChild } from '@angular/core';
import { Scene, SceneConfig } from './services/scene';
import { ScenePanel } from './scene-panel/scene-panel';
import { SceneData } from './render.worker';

const ROWS_PER_CHUNK = 5;

@Component({
  selector: 'app-root',
  imports: [ScenePanel],
  templateUrl: './app.html',
  styleUrl: './app.css',
  host: {
    "(window:resize)": "onResize()"
  }
})
export class App {
  private canvas = viewChild.required<ElementRef<HTMLCanvasElement>>('raytracerCanvas');
  private windowSize = signal({ width: window.innerWidth, height: window.innerHeight });
  private renderTick = signal(0);
  protected rendering = signal(false);
  private workers: Worker[] = [];
  private scene = inject(Scene);

  constructor() {
    const numWorkers = (navigator.hardwareConcurrency || 4) + 1;
    for (let i = 0; i < numWorkers; i++) {
      this.workers.push(new Worker(new URL('./render.worker', import.meta.url), { type: 'module' }));
    }

    effect(() => {
      const canvas = this.canvas();
      if (!canvas) return;

      const { width, height } = this.windowSize();
      this.renderTick();
      canvas.nativeElement.width = width;
      canvas.nativeElement.height = height;

      const sceneConfig = untracked(() => this.scene.scene());
      this.renderParallel(width, height, canvas.nativeElement, sceneConfig);
    });
  }

  onResize() {
    this.windowSize.set({ width: window.innerWidth, height: window.innerHeight });
  }

  render() {
    this.renderTick.update(t => t + 1);
  }

  saveImage() {
    const canvas = this.canvas().nativeElement;
    canvas.toBlob(blob => {
      if (!blob) return;
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `raytracer-${new Date().toISOString().replace(/[:.]/g, '-')}.png`;
      a.click();
      URL.revokeObjectURL(url);
    }, 'image/png');
  }

  private renderParallel(width: number, height: number, canvas: HTMLCanvasElement, sceneConfig: SceneConfig) {
    let gl = (canvas as any).__gl as WebGL2RenderingContext;
    let program = (canvas as any).__program as WebGLProgram;
    let tex = (canvas as any).__tex as WebGLTexture;

    if (!gl) {
      gl = canvas.getContext('webgl2', {
        antialias: false,
        depth: false,
        premultipliedAlpha: false,
        preserveDrawingBuffer: true,
        // @ts-ignore
        colorSpace: 'display-p3'
      }) as WebGL2RenderingContext;

      gl.getExtension('EXT_color_buffer_float');

      const vs = gl.createShader(gl.VERTEX_SHADER)!;
      gl.shaderSource(vs, `#version 300 es
        in vec2 a_position;
        in vec2 a_texCoord;
        out vec2 v_texCoord;
        void main() {
          gl_Position = vec4(a_position, 0.0, 1.0);
          v_texCoord = a_texCoord;
        }
      `);
      gl.compileShader(vs);

      const fs = gl.createShader(gl.FRAGMENT_SHADER)!;
      gl.shaderSource(fs, `#version 300 es
        precision highp float;
        uniform sampler2D u_image;
        in vec2 v_texCoord;
        out vec4 outColor;
        void main() {
          vec4 color = texture(u_image, vec2(v_texCoord.x, v_texCoord.y));
          outColor = color;
        }
      `);
      gl.compileShader(fs);

      program = gl.createProgram()!;
      gl.attachShader(program, vs);
      gl.attachShader(program, fs);
      gl.linkProgram(program);

      const posBuf = gl.createBuffer();
      gl.bindBuffer(gl.ARRAY_BUFFER, posBuf);
      gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
        -1, -1, 0, 1,
        1, -1, 1, 1,
        -1, 1, 0, 0,
        1, 1, 1, 0,
      ]), gl.STATIC_DRAW);

      tex = gl.createTexture()!;
      gl.bindTexture(gl.TEXTURE_2D, tex);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

      (canvas as any).__gl = gl;
      (canvas as any).__program = program;
      (canvas as any).__tex = tex;
    }

    gl.viewport(0, 0, width, height);
    gl.useProgram(program);

    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA32F, width, height, 0, gl.RGBA, gl.FLOAT, null);

    const posLoc = gl.getAttribLocation(program, 'a_position');
    const texLoc = gl.getAttribLocation(program, 'a_texCoord');

    gl.enableVertexAttribArray(posLoc);
    gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 16, 0);
    gl.enableVertexAttribArray(texLoc);
    gl.vertexAttribPointer(texLoc, 2, gl.FLOAT, false, 16, 8);

    const sphereData = this.scene.buildSphereData(sceneConfig.spheres);
    const triangleData = this.scene.buildTriangleData(sceneConfig.triangles, sceneConfig.objects, sceneConfig.terrains);
    const lightData = this.scene.buildLightData(sceneConfig.lights);
    const diffuseIntensity = sceneConfig.diffuseIntensity;
    const skybox = sceneConfig.skybox;
    const skyboxPixels = skybox?.pixels ?? new Float32Array(0);
    const skyboxWidth = skybox?.width ?? 0;
    const skyboxHeight = skybox?.height ?? 0;
    const skyboxBrightness = skybox?.brightness ?? 1;
    const cloudData = this.scene.buildCloudData(sceneConfig.clouds);
    const samplesPerAxis = Math.max(1, sceneConfig.samplesPerAxis | 0);

    const chunks: { startRow: number; endRow: number }[] = [];
    for (let row = 0; row < height; row += ROWS_PER_CHUNK) {
      chunks.push({ startRow: row, endRow: Math.min(row + ROWS_PER_CHUNK, height) });
    }

    let completed = 0;
    const total = chunks.length;
    this.rendering.set(true);

    const dispatch = (worker: Worker) => {
      const chunk = chunks.pop();
      if (!chunk) return;

      worker.onmessage = ({ data }: MessageEvent<{ startRow: number; endRow: number; pixels: Float32Array }>) => {
        gl.bindTexture(gl.TEXTURE_2D, tex);
        gl.texSubImage2D(gl.TEXTURE_2D, 0, 0, data.startRow, width, data.endRow - data.startRow, gl.RGBA, gl.FLOAT, data.pixels);

        gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

        completed++;

        if (completed === total) {
          this.rendering.set(false);
        } else {
          dispatch(worker);
        }
      };

      const sceneData: SceneData = { width, height, startRow: chunk.startRow, endRow: chunk.endRow, sphereData, triangleData, lightData, diffuseIntensity, skyboxPixels, skyboxWidth, skyboxHeight, skyboxBrightness, cloudData, samplesPerAxis };

      worker.postMessage(sceneData);
    };

    for (const worker of this.workers) {
      dispatch(worker);
    }
  }
}
