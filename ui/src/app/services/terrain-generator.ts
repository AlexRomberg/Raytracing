import { Injectable } from '@angular/core';
import init, { generate_terrain } from 'raytracer';
import type { ParsedMesh } from './obj-loader';
import type { Color, Vec3 } from './scene';

export interface TerrainParams {
  gridSize: number;
  width: number;
  depth: number;
  heightScale: number;
  octaves: number;
  persistence: number;
  lacunarity: number;
  seed: number;
}

@Injectable({ providedIn: 'root' })
export class TerrainGenerator {
  private ready: Promise<void> | null = null;

  private ensureReady(): Promise<void> {
    if (!this.ready) {
      this.ready = init({ module_or_path: './raytracer_bg.wasm' }).then(() => { });
    }
    return this.ready;
  }

  async generate(params: TerrainParams): Promise<ParsedMesh> {
    await this.ensureReady();
    const data = generate_terrain(
      params.gridSize,
      params.width,
      params.depth,
      params.heightScale,
      params.octaves,
      params.persistence,
      params.lacunarity,
      params.seed >>> 0,
    );
    return unpackTerrainMesh(data);
  }
}

function unpackTerrainMesh(data: Float32Array): ParsedMesh {
  const vc = data[0] | 0;
  const fc = data[1] | 0;
  const vertices: Vec3[] = new Array(vc);
  for (let i = 0; i < vc; i++) {
    const o = 2 + i * 3;
    vertices[i] = { x: data[o], y: data[o + 1], z: data[o + 2] };
  }
  const facesOff = 2 + vc * 3;
  const faces: [number, number, number][] = new Array(fc);
  for (let i = 0; i < fc; i++) {
    const o = facesOff + i * 3;
    faces[i] = [data[o] | 0, data[o + 1] | 0, data[o + 2] | 0];
  }
  const colorsOff = facesOff + fc * 3;
  const faceColors: Color[] = new Array(fc);
  for (let i = 0; i < fc; i++) {
    const o = colorsOff + i * 3;
    faceColors[i] = { r: data[o], g: data[o + 1], b: data[o + 2] };
  }
  return { vertices, faces, faceColors };
}
