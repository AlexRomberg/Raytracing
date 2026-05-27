import { Component, computed, inject, input, output, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CloudConfig, Color, Scene, TerrainConfig, Vec3 } from '../services/scene';
import { ObjLoader } from '../services/obj-loader';
import { TerrainGenerator } from '../services/terrain-generator';
import { Vec3Input } from '../components/vec3-input/vec3-input';
import { ColorInput } from '../components/color-input/color-input';

@Component({
    selector: 'app-scene-panel',
    imports: [FormsModule, Vec3Input, ColorInput],
    templateUrl: './scene-panel.html',
    styleUrl: './scene-panel.css',
})
export class ScenePanel {
    protected scene = inject(Scene);
    private objLoader = inject(ObjLoader);
    private terrainGenerator = inject(TerrainGenerator);
    protected open = signal(false);
    protected generatingTerrain = signal<number | null>(null);
    public rendering = input.required<boolean>();
    public render = output();
    public save = output();

    protected sceneConfig = this.scene.scene;
    protected spheres = computed(() => this.sceneConfig().spheres);
    protected lights = computed(() => this.sceneConfig().lights);
    protected triangles = computed(() => this.sceneConfig().triangles);
    protected objects = computed(() => this.sceneConfig().objects);
    protected terrains = computed(() => this.sceneConfig().terrains);
    protected clouds = computed(() => this.sceneConfig().clouds);
    protected diffuseIntensity = computed(() => this.sceneConfig().diffuseIntensity);
    protected samplesPerAxis = computed(() => this.sceneConfig().samplesPerAxis);
    protected skybox = computed(() => this.sceneConfig().skybox);

    toggle() {
        this.open.update(v => !v);
    }

    onDiffuseChange(value: number) {
        this.scene.update({ diffuseIntensity: value });
    }

    onSamplesPerAxisChange(value: number) {
        this.scene.update({ samplesPerAxis: Math.max(1, value | 0) });
    }

    onSphereChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateSphere(index, { [field]: value });
    }

    onTriangleChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateTriangle(index, { [field]: value });
    }

    onLightChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateLight(index, { [field]: value });
    }

    onObjectChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateObject(index, { [field]: value });
    }

    addSphere() {
        this.scene.addSphere();
    }

    removeSphere(index: number) {
        this.scene.removeSphere(index);
    }

    addTriangle() {
        this.scene.addTriangle();
    }

    removeTriangle(index: number) {
        this.scene.removeTriangle(index);
    }

    addLight() {
        this.scene.addLight();
    }

    removeLight(index: number) {
        this.scene.removeLight(index);
    }

    addObject() {
        this.scene.addObject();
    }

    removeObject(index: number) {
        this.scene.removeObject(index);
    }

    async onObjFileSelected(index: number, event: Event) {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;
        const text = await file.text();
        const mesh = this.objLoader.parse(text);
        this.scene.updateObject(index, { mesh, name: file.name });
    }

    async onSkyboxFileSelected(event: Event) {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;
        const url = URL.createObjectURL(file);
        try {
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
            this.scene.setSkybox({
                pixels,
                width: canvas.width,
                height: canvas.height,
                brightness: this.skybox()?.brightness ?? 1,
            });
        } finally {
            URL.revokeObjectURL(url);
        }
    }

    onSkyboxBrightnessChange(value: number) {
        this.scene.updateSkyboxBrightness(value);
    }

    clearSkybox() {
        this.scene.setSkybox(null);
    }

    addTerrain() {
        this.scene.addTerrain();
    }

    removeTerrain(index: number) {
        this.scene.removeTerrain(index);
    }

    onTerrainChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateTerrain(index, { [field]: value } as Partial<TerrainConfig>);
    }

    randomizeTerrainSeed(index: number) {
        const seed = Math.floor(Math.random() * 0xffffffff) >>> 0;
        this.scene.updateTerrain(index, { seed });
    }

    addCloud() {
        this.scene.addCloud();
    }

    removeCloud(index: number) {
        this.scene.removeCloud(index);
    }

    onCloudChange(index: number, field: string, value: number | string | Color | Vec3) {
        this.scene.updateCloud(index, { [field]: value } as Partial<CloudConfig>);
    }

    randomizeCloudSeed(index: number) {
        const seed = Math.floor(Math.random() * 0xffffffff) >>> 0;
        this.scene.updateCloud(index, { seed });
    }

    async generateTerrain(index: number, terrain: TerrainConfig) {
        this.generatingTerrain.set(index);
        try {
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
            this.scene.updateTerrain(index, { mesh });
        } finally {
            this.generatingTerrain.set(null);
        }
    }
}
