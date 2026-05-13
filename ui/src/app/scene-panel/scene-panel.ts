import { Component, computed, inject, input, output, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Color, Scene, Vec3 } from '../services/scene';
import { ObjLoader } from '../services/obj-loader';
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
    protected open = signal(false);
    public rendering = input.required<boolean>();
    public render = output();

    protected sceneConfig = this.scene.scene;
    protected spheres = computed(() => this.sceneConfig().spheres);
    protected lights = computed(() => this.sceneConfig().lights);
    protected triangles = computed(() => this.sceneConfig().triangles);
    protected objects = computed(() => this.sceneConfig().objects);
    protected diffuseIntensity = computed(() => this.sceneConfig().diffuseIntensity);

    toggle() {
        this.open.update(v => !v);
    }

    onDiffuseChange(value: number) {
        this.scene.update({ diffuseIntensity: value });
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
}
