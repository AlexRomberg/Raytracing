import { Injectable } from '@angular/core';
import { Vec3 } from './scene';

export interface ParsedMesh {
  vertices: Vec3[];
  faces: [number, number, number][];
}

@Injectable({ providedIn: 'root' })
export class ObjLoader {
  parse(source: string): ParsedMesh {
    const vertices: Vec3[] = [];
    const faces: [number, number, number][] = [];

    const lines = source.split(/\r?\n/);
    for (const rawLine of lines) {
      const line = rawLine.trim();
      if (line.length === 0 || line.startsWith('#')) continue;

      const parts = line.split(/\s+/);
      const tag = parts[0];

      if (tag === 'v') {
        const x = parseFloat(parts[1]);
        const y = parseFloat(parts[2]);
        const z = parseFloat(parts[3]);
        if (Number.isFinite(x) && Number.isFinite(y) && Number.isFinite(z)) {
          vertices.push({ x, y, z });
        }
      } else if (tag === 'f') {
        const idx: number[] = [];
        for (let i = 1; i < parts.length; i++) {
          const token = parts[i].split('/')[0];
          const n = parseInt(token, 10);
          if (!Number.isNaN(n)) {
            idx.push(n > 0 ? n - 1 : vertices.length + n);
          }
        }
        if (idx.length < 3) continue;
        for (let i = 1; i < idx.length - 1; i++) {
          faces.push([idx[0], idx[i], idx[i + 1]]);
        }
      }
    }

    return { vertices, faces };
  }
}
