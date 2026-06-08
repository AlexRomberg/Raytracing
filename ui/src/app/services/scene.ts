import { Injectable, signal } from '@angular/core';
import { ObjLoader, type ParsedMesh } from './obj-loader';
import { TerrainGenerator } from './terrain-generator';

export interface Vec3 {
  x: number;
  y: number;
  z: number;
}

export interface Color {
  r: number;
  g: number;
  b: number;
}

export interface SphereConfig {
  name: string;
  center: Vec3;
  radius: number;
  color: Color;
  shininess: number;
  diffuse: number;
  specular: number;
  materialType?: MaterialType;
}

export interface LightConfig {
  name: string;
  center: Vec3;
  color: Color;
}

export interface TriangleConfig {
  name: string;
  pointA: Vec3;
  pointB: Vec3;
  pointC: Vec3;
  color: Color;
  shininess: number;
  diffuse: number;
  specular: number;
  materialType?: MaterialType;
}

export interface ObjectConfig {
  name: string;
  mesh: ParsedMesh | null;
  scale: number;
  offset: Vec3;
  color: Color;
  shininess: number;
  diffuse: number;
  specular: number;
  materialType?: MaterialType;
}

export interface TerrainConfig {
  name: string;
  gridSize: number;
  width: number;
  depth: number;
  heightScale: number;
  octaves: number;
  persistence: number;
  lacunarity: number;
  seed: number;
  offset: Vec3;
  shininess: number;
  diffuse: number;
  specular: number;
  materialType?: MaterialType;
  mesh: ParsedMesh | null;
}

enum MaterialType {
  BlinnPhong = 0,
  Metal = 1,
  Dielectric = 2,
}

const DEFAULT_SHININESS = 32;
const DEFAULT_DIFFUSE = 1;
const DEFAULT_SPECULAR = 0.7;

export interface SkyboxConfig {
  pixels: Float32Array;
  width: number;
  height: number;
  brightness: number;
}

export async function loadSkyboxFromUrl(url: string, brightness = 1): Promise<SkyboxConfig> {
  const img = new Image();
  img.src = url;
  await img.decode();
  const canvas = document.createElement('canvas');
  canvas.width = img.naturalWidth;
  canvas.height = img.naturalHeight;
  const ctx = canvas.getContext('2d')!;
  ctx.drawImage(img, 0, 0);
  const { data } = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const pixels = new Float32Array(canvas.width * canvas.height * 3);
  for (let i = 0, j = 0; i < data.length; i += 4, j += 3) {
    pixels[j] = data[i] / 255;
    pixels[j + 1] = data[i + 1] / 255;
    pixels[j + 2] = data[i + 2] / 255;
  }
  return { pixels, width: canvas.width, height: canvas.height, brightness };
}

export interface CloudConfig {
  name: string;
  center: Vec3;
  size: Vec3;
  density: number;
  noiseScale: number;
  octaves: number;
  seed: number;
  color: Color;
}

export interface SceneConfig {
  diffuseIntensity: number;
  samplesPerAxis: number;
  spheres: SphereConfig[];
  objects: ObjectConfig[];
  terrains: TerrainConfig[];
  triangles: TriangleConfig[];
  lights: LightConfig[];
  clouds: CloudConfig[];
  skybox: SkyboxConfig | null;
}

const DEFAULT_SCENE: SceneConfig = {
  diffuseIntensity: 0.1,
  samplesPerAxis: 1,
  spheres: [
    {
      name: 'Glass Ball',
      center: { x: 0, y: 50, z: 400 },
      radius: 120,
      color: { r: 1.0, g: 1.0, b: 1.0 },
      shininess: DEFAULT_SHININESS,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.Dielectric,
    },
    {
      name: 'Moon',
      center: { x: -1000, y: 1000, z: 3000 },
      radius: 90,
      color: { r: 1.0, g: 1.0, b: 1.0 },
      shininess: 30,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
  ],
  triangles: [
    {
      name: 'Lake right',
      pointA: { x: -100, y: -120, z: 0 },
      pointB: { x: 100, y: -120, z: 0 },
      pointC: { x: 100, y: -120, z: 400 },
      color: { r: 1.0, g: 1.0, b: 1.0 },
      shininess: DEFAULT_SHININESS,
      diffuse: 0,
      specular: 1.0,
      materialType: MaterialType.Metal,
    },
    {
      name: 'Lake left',
      pointA: { x: -100, y: -120, z: 0 },
      pointB: { x: 100, y: -120, z: 400 },
      pointC: { x: -100, y: -120, z: 400 },
      color: { r: 1.0, g: 1.0, b: 1.0 },
      shininess: DEFAULT_SHININESS,
      diffuse: 0,
      specular: 1.0,
      materialType: MaterialType.Metal,
    },
    {
      name: 'Cube Left - Inner Wall 1',
      pointA: { x: -100, y: -210, z: 0 },
      pointB: { x: -100, y: -110, z: 0 },
      pointC: { x: -100, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Left - Inner Wall 2',
      pointA: { x: -100, y: -210, z: 0 },
      pointB: { x: -100, y: -110, z: 400 },
      pointC: { x: -100, y: -210, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Left - Top 1',
      pointA: { x: -500, y: -110, z: 0 },
      pointB: { x: -100, y: -110, z: 0 },
      pointC: { x: -100, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Left - Top 2',
      pointA: { x: -500, y: -110, z: 0 },
      pointB: { x: -100, y: -110, z: 400 },
      pointC: { x: -500, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Right - Inner Wall 1',
      pointA: { x: 100, y: -210, z: 0 },
      pointB: { x: 100, y: -110, z: 400 },
      pointC: { x: 100, y: -110, z: 0 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Right - Inner Wall 2',
      pointA: { x: 100, y: -210, z: 0 },
      pointB: { x: 100, y: -210, z: 400 },
      pointC: { x: 100, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Right - Top 1',
      pointA: { x: 100, y: -110, z: 0 },
      pointB: { x: 500, y: -110, z: 0 },
      pointC: { x: 500, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
    {
      name: 'Cube Right - Top 2',
      pointA: { x: 100, y: -110, z: 0 },
      pointB: { x: 500, y: -110, z: 400 },
      pointC: { x: 100, y: -110, z: 400 },
      color: { r: 0.7, g: 0.6, b: 0.4 },
      shininess: 3,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
    },
  ],
  lights: [
    { name: 'Sun', center: { x: 1000, y: 50, z: 1000 }, color: { r: 1, g: 0.65, b: 0.55 } },
    { name: 'Moon', center: { x: -100, y: 1000, z: 0 }, color: { r: 0.15, g: 0.2, b: 0.35 } },
  ],
  objects: [
    {
      color: { r: 0.2, g: 0.2, b: 0.2 },
      name: 'Lantern left',
      mesh: null,
      offset: { x: -150, y: -110, z: 300 },
      scale: 60,
      shininess: 0.1,
      diffuse: 0.5,
      specular: 0.4,
      materialType: MaterialType.BlinnPhong
    },
    {
      color: { r: 0.2, g: 0.2, b: 0.2 },
      name: 'Lantern right',
      mesh: null,
      offset: { x: 150, y: -110, z: 300 },
      scale: 60,
      shininess: 0.1,
      diffuse: 0.5,
      specular: 0.4,
      materialType: MaterialType.BlinnPhong
    },
  ],
  terrains: [
    {
      name: 'Mountain range',
      gridSize: 150,
      width: 2000,
      depth: 390,
      heightScale: 400,
      octaves: 5,
      persistence: 0.5,
      lacunarity: 2.5,
      seed: 770983921,
      offset: { x: 0, y: -150, z: 700 },
      shininess: 5,
      diffuse: DEFAULT_DIFFUSE,
      specular: DEFAULT_SPECULAR,
      materialType: MaterialType.BlinnPhong,
      mesh: null,
    },
  ],
  clouds: [
    {
      name: 'Cloud right',
      center: { x: 1000, y: 300, z: 1500 },
      size: { x: 1000, y: 600, z: 600 },
      density: 10,
      noiseScale: 0.01,
      octaves: 5,
      seed: 2396838568,
      color: { r: 0.6, g: 0.5, b: 0.5 },
    },
    {
      name: 'Cloud left',
      center: { x: -1000, y: 100, z: 1500 },
      size: { x: 1300, y: 600, z: 600 },
      density: 10,
      noiseScale: 0.01,
      octaves: 5,
      seed: 2093842823,
      color: { r: 0.6, g: 0.5, b: 0.5 },
    }
  ],
  skybox: null,
};

@Injectable({
  providedIn: 'root',
})
export class Scene {
  scene = signal<SceneConfig>(structuredClone(DEFAULT_SCENE));

  readonly assetsRevision = signal(0);

  constructor(
    private terrainGenerator: TerrainGenerator,
    private objLoader: ObjLoader,
  ) {
    this.initializeTerrainMeshes();
    this.initializeDefaultObjectMeshes();
    this.initializeDefaultSkybox();
  }

  private async initializeDefaultObjectMeshes(): Promise<void> {
    const objects = this.scene().objects;
    if (!objects.some(o => !o.mesh)) return;
    try {
      const text = await fetch('/Lantern.obj').then(r => r.text());
      const mesh = this.objLoader.parse(text);
      this.scene().objects.forEach((obj, i) => {
        if (!obj.mesh) this.updateObject(i, { mesh });
      });
      this.assetsRevision.update(r => r + 1);
    } catch (err) {
      console.error('Failed to load default object mesh /Lantern.obj', err);
    }
  }

  private async initializeDefaultSkybox(): Promise<void> {
    if (this.scene().skybox) return;
    try {
      const skybox = await loadSkyboxFromUrl('/skytexture.jpg');
      this.setSkybox(skybox);
      this.assetsRevision.update(r => r + 1);
    } catch (err) {
      console.error('Failed to load default skybox /skytexture.jpg', err);
    }
  }

  private async initializeTerrainMeshes(): Promise<void> {
    const currentScene = this.scene();
    for (let i = 0; i < currentScene.terrains.length; i++) {
      const terrain = currentScene.terrains[i];
      if (!terrain.mesh) {
        const mesh = await this.terrainGenerator.generate({
          gridSize: terrain.gridSize,
          width: terrain.width,
          depth: terrain.depth,
          heightScale: terrain.heightScale,
          octaves: terrain.octaves,
          persistence: terrain.persistence,
          lacunarity: terrain.lacunarity,
          seed: terrain.seed,
        });
        this.updateTerrain(i, { mesh });
        this.assetsRevision.update(r => r + 1);
      }
    }
  }

  update(partial: Partial<SceneConfig>) {
    this.scene.update(s => ({ ...s, ...partial }));
  }

  updateSphere(index: number, partial: Partial<SphereConfig>) {
    this.scene.update(s => {
      const spheres = s.spheres.map((sp, i) => i === index ? { ...sp, ...partial } : sp);
      return { ...s, spheres };
    });
  }

  addSphere() {
    this.scene.update(s => ({
      ...s,
      spheres: [
        ...s.spheres,
        {
          name: `Sphere ${s.spheres.length + 1}`,
          center: { x: 800, y: 400, z: 300 },
          radius: 100,
          color: { r: 1, g: 1, b: 1 },
          shininess: DEFAULT_SHININESS,
          diffuse: DEFAULT_DIFFUSE,
          specular: DEFAULT_SPECULAR,
        },
      ],
    }));
  }

  removeSphere(index: number) {
    this.scene.update(s => ({
      ...s,
      spheres: s.spheres.filter((_, i) => i !== index),
    }));
  }

  updateTriangle(index: number, partial: Partial<TriangleConfig>) {
    this.scene.update(s => {
      const triangles = s.triangles.map((t, i) => i === index ? { ...t, ...partial } : t);
      return { ...s, triangles };
    });
  }

  addTriangle() {
    this.scene.update(s => ({
      ...s,
      triangles: [
        ...s.triangles,
        {
          name: `Triangle ${s.triangles.length + 1}`,
          pointA: { x: 700, y: 400, z: 200 },
          pointB: { x: 900, y: 400, z: 200 },
          pointC: { x: 800, y: 400, z: 400 },
          color: { r: 1, g: 1, b: 1 },
          shininess: DEFAULT_SHININESS,
          diffuse: DEFAULT_DIFFUSE,
          specular: DEFAULT_SPECULAR,
        },
      ],
    }));
  }

  removeTriangle(index: number) {
    this.scene.update(s => ({
      ...s,
      triangles: s.triangles.filter((_, i) => i !== index),
    }));
  }

  updateLight(index: number, partial: Partial<LightConfig>) {
    this.scene.update(s => {
      const lights = s.lights.map((l, i) => i === index ? { ...l, ...partial } : l);
      return { ...s, lights };
    });
  }

  addLight() {
    this.scene.update(s => ({
      ...s,
      lights: [...s.lights, { name: `Light ${s.lights.length + 1}`, center: { x: 400, y: 400, z: -200 }, color: { r: 1, g: 1, b: 1 } }],
    }));
  }

  removeLight(index: number) {
    this.scene.update(s => ({
      ...s,
      lights: s.lights.filter((_, i) => i !== index),
    }));
  }

  updateObject(index: number, partial: Partial<ObjectConfig>) {
    this.scene.update(s => {
      const objects = s.objects.map((sp, i) => i === index ? { ...sp, ...partial } : sp);
      return { ...s, objects };
    });
  }

  addObject() {
    this.scene.update(s => ({
      ...s,
      objects: [
        ...s.objects,
        {
          name: `Object ${s.objects.length + 1}`,
          mesh: null,
          scale: 100,
          offset: { x: 0, y: 0, z: 400 },
          color: { r: 1, g: 1, b: 1 },
          shininess: DEFAULT_SHININESS,
          diffuse: DEFAULT_DIFFUSE,
          specular: DEFAULT_SPECULAR,
        },
      ],
    }));
  }

  removeObject(index: number) {
    this.scene.update(s => ({
      ...s,
      objects: s.objects.filter((_, i) => i !== index),
    }));
  }

  updateTerrain(index: number, partial: Partial<TerrainConfig>) {
    this.scene.update(s => {
      const terrains = s.terrains.map((t, i) => i === index ? { ...t, ...partial } : t);
      return { ...s, terrains };
    });
  }

  addTerrain() {
    this.scene.update(s => ({
      ...s,
      terrains: [
        ...s.terrains,
        {
          name: `Terrain ${s.terrains.length + 1}`,
          gridSize: 128,
          width: 1200,
          depth: 1200,
          heightScale: 300,
          octaves: 5,
          persistence: 0.5,
          lacunarity: 2.0,
          seed: Math.floor(Math.random() * 0xffffffff) >>> 0,
          offset: { x: 0, y: -300, z: 700 },
          shininess: DEFAULT_SHININESS,
          diffuse: DEFAULT_DIFFUSE,
          specular: DEFAULT_SPECULAR,
          mesh: null,
        },
      ],
    }));

    const newTerrainIndex = this.scene().terrains.length - 1;
    const newTerrain = this.scene().terrains[newTerrainIndex];
    this.terrainGenerator.generate({
      gridSize: newTerrain.gridSize,
      width: newTerrain.width,
      depth: newTerrain.depth,
      heightScale: newTerrain.heightScale,
      octaves: newTerrain.octaves,
      persistence: newTerrain.persistence,
      lacunarity: newTerrain.lacunarity,
      seed: newTerrain.seed,
    }).then(mesh => {
      this.updateTerrain(newTerrainIndex, { mesh });
    });
  }

  removeTerrain(index: number) {
    this.scene.update(s => ({
      ...s,
      terrains: s.terrains.filter((_, i) => i !== index),
    }));
  }

  updateCloud(index: number, partial: Partial<CloudConfig>) {
    this.scene.update(s => {
      const clouds = s.clouds.map((c, i) => i === index ? { ...c, ...partial } : c);
      return { ...s, clouds };
    });
  }

  addCloud() {
    this.scene.update(s => ({
      ...s,
      clouds: [
        ...s.clouds,
        {
          name: `Cloud ${s.clouds.length + 1}`,
          center: { x: 0, y: 300, z: 1500 },
          size: { x: 2000, y: 400, z: 800 },
          density: 4.0,
          noiseScale: 0.004,
          octaves: 4,
          seed: Math.floor(Math.random() * 0xffffffff) >>> 0,
          color: { r: 1, g: 1, b: 1 },
        },
      ],
    }));
  }

  removeCloud(index: number) {
    this.scene.update(s => ({
      ...s,
      clouds: s.clouds.filter((_, i) => i !== index),
    }));
  }

  buildCloudData(clouds: CloudConfig[]): Float32Array {
    const data = new Float32Array(clouds.length * 13);
    for (let i = 0; i < clouds.length; i++) {
      const c = clouds[i];
      const o = i * 13;
      data[o] = c.center.x;
      data[o + 1] = c.center.y;
      data[o + 2] = -c.center.z;
      data[o + 3] = c.size.x;
      data[o + 4] = c.size.y;
      data[o + 5] = c.size.z;
      data[o + 6] = c.density;
      data[o + 7] = c.noiseScale;
      data[o + 8] = c.octaves;
      data[o + 9] = c.seed;
      data[o + 10] = c.color.r;
      data[o + 11] = c.color.g;
      data[o + 12] = c.color.b;
    }
    return data;
  }

  public buildSphereData(spheres: SphereConfig[]): Float32Array {
    const data = new Float32Array(spheres.length * 11);
    for (let i = 0; i < spheres.length; i++) {
      const s = spheres[i];
      const o = i * 11;
      data[o] = s.center.x; data[o + 1] = s.center.y; data[o + 2] = -s.center.z;
      data[o + 3] = s.radius;
      data[o + 4] = s.color.r; data[o + 5] = s.color.g; data[o + 6] = s.color.b;
      data[o + 7] = s.shininess;
      data[o + 8] = s.diffuse;
      data[o + 9] = s.specular;
      data[o + 10] = s.materialType ?? 0;
    }
    return data;
  }

  buildTriangleData(
    triangles: TriangleConfig[],
    objects: ObjectConfig[] = [],
    terrains: TerrainConfig[] = [],
  ): Float32Array {
    let meshTriCount = 0;
    for (const obj of objects) {
      if (obj.mesh) meshTriCount += obj.mesh.faces.length;
    }
    for (const t of terrains) {
      if (t.mesh) meshTriCount += t.mesh.faces.length;
    }
    const total = triangles.length + meshTriCount;
    const data = new Float32Array(total * 16);

    for (let i = 0; i < triangles.length; i++) {
      const t = triangles[i];
      const o = i * 16;
      data[o] = t.pointA.x;
      data[o + 1] = t.pointA.y;
      data[o + 2] = -t.pointA.z;

      data[o + 3] = t.pointB.x;
      data[o + 4] = t.pointB.y;
      data[o + 5] = -t.pointB.z;

      data[o + 6] = t.pointC.x;
      data[o + 7] = t.pointC.y;
      data[o + 8] = -t.pointC.z;

      data[o + 9] = t.color.r;
      data[o + 10] = t.color.g;
      data[o + 11] = t.color.b;

      data[o + 12] = t.shininess;
      data[o + 13] = t.diffuse;
      data[o + 14] = t.specular;
      data[o + 15] = t.materialType ?? 0;
    }

    let cursor = triangles.length;
    const writeMesh = (
      mesh: ParsedMesh,
      scale: number,
      offset: Vec3,
      fallback: Color,
      shininess: number,
      diffuse: number,
      specular: number,
      materialType: number,
    ) => {
      const { vertices, faces, faceColors } = mesh;
      for (let fi = 0; fi < faces.length; fi++) {
        const [ia, ib, ic] = faces[fi];
        const a = vertices[ia];
        const b = vertices[ib];
        const c = vertices[ic];
        const tc = faceColors?.[fi] ?? fallback;
        const o = cursor * 16;
        data[o] = a.x * scale + offset.x;
        data[o + 1] = a.y * scale + offset.y;
        data[o + 2] = -(a.z * scale + offset.z);

        data[o + 3] = b.x * scale + offset.x;
        data[o + 4] = b.y * scale + offset.y;
        data[o + 5] = -(b.z * scale + offset.z);

        data[o + 6] = c.x * scale + offset.x;
        data[o + 7] = c.y * scale + offset.y;
        data[o + 8] = -(c.z * scale + offset.z);

        data[o + 9] = tc.r;
        data[o + 10] = tc.g;
        data[o + 11] = tc.b;

        data[o + 12] = shininess;
        data[o + 13] = diffuse;
        data[o + 14] = specular;
        data[o + 15] = materialType;
        cursor++;
      }
    };

    for (const obj of objects) {
      if (!obj.mesh) continue;
      writeMesh(
        obj.mesh,
        obj.scale,
        obj.offset,
        obj.color,
        obj.shininess,
        obj.diffuse,
        obj.specular,
        obj.materialType ?? 0,
      );
    }

    for (const terrain of terrains) {
      if (!terrain.mesh) continue;
      writeMesh(
        terrain.mesh,
        1,
        terrain.offset,
        { r: 1, g: 1, b: 1 },
        terrain.shininess,
        terrain.diffuse,
        terrain.specular,
        terrain.materialType ?? 0,
      );
    }

    return data;
  }

  setSkybox(skybox: SkyboxConfig | null) {
    this.scene.update(s => ({ ...s, skybox }));
  }

  updateSkyboxBrightness(brightness: number) {
    this.scene.update(s => s.skybox ? { ...s, skybox: { ...s.skybox, brightness } } : s);
  }

  public buildLightData(lights: LightConfig[]): Float32Array {
    const data = new Float32Array(lights.length * 6);
    for (let i = 0; i < lights.length; i++) {
      const l = lights[i];
      const o = i * 6;
      data[o] = l.center.x;
      data[o + 1] = l.center.y;
      data[o + 2] = -l.center.z;

      data[o + 3] = l.color.r;
      data[o + 4] = l.color.g;
      data[o + 5] = l.color.b;
    }
    return data;
  }
}
