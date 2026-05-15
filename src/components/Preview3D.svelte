<!-- Copyright (c) 2026 SimplePicture3D Contributors
     SPDX-License-Identifier: MIT -->
<script lang="ts">
  /**
   * Preview3D — Three.js 3D preview (UI-501–UI-506).
   *
   * Sprint A: the 2.5D mesh preview (point cloud + triangulated wireframe/solid via
   * `get_mesh_data`) has been retired. The scene, camera, lights, grid, orbit controls,
   * and synthetic-data performance test remain in place. Live point cloud rendering will
   * be re-wired to the 3D volumetric pipeline (`generate_point_cloud`, ADR-012) plus a
   * blank-envelope wireframe in Sprint B / Sprint C.
   */
  import { onMount, onDestroy } from "svelte";
  import * as THREE from "three";
  import { OrbitControls } from "three/addons/controls/OrbitControls.js";

  let container: HTMLDivElement;
  /** Target dimensions in mm from App (legacy 2.5D scaling). Retained as props until Sprint C
   *  replaces them with blank-envelope dimensions. The values are surfaced as `data-` attributes
   *  on the root div so they are observable in the DOM and so the Svelte compiler does not
   *  warn about unused exports during Sprint A. */
  export let targetWidthMm: number | undefined = undefined;
  export let targetHeightMm: number | undefined = undefined;

  let scene: THREE.Scene;
  let camera: THREE.PerspectiveCamera;
  let renderer: THREE.WebGLRenderer;
  let controls: OrbitControls;
  let gridHelper: THREE.GridHelper;
  /** JR1-505: light refs for intensity controls (set in onMount) */
  let ambientLight: THREE.AmbientLight | undefined;
  let directionalLight: THREE.DirectionalLight | undefined;
  /** Active rendered point cloud (synthetic from perf test today; real volumetric in Sprint B/C). */
  let pointCloud: THREE.Points | null = null;
  let animationId: number | null = null;
  let resizeObserver: ResizeObserver | null = null;

  /** UI-503: loading and error state */
  let meshLoading = false;
  let meshError = "";

  /** JR1-501: mesh bounding box for camera presets (set when mesh loaded). */
  let meshBox: THREE.Box3 | null = null;

  /** JR1-503: mesh statistics (vertex count, bounds in mm); cleared when no mesh. */
  let meshStats: { vertexCount: number; minX: number; maxX: number; minY: number; maxY: number; minZ: number; maxZ: number } | null = null;

  /** JR1-505: lighting controls (defaults match initial scene lights). */
  let ambientIntensity = 0.6;
  let directionalIntensity = 0.8;
  /** JR1-504: last performance test result (100K, 500K, 1M FPS). */
  let perfResult: { "100K": number; "500K": number; "1M": number } | null = null;
  /** JR1-504: in-progress perf test state. */
  let perfTest: {
    index: number;
    sizes: number[];
    startTime: number;
    frameCount: number;
    results: Record<string, number>;
    savedPointCloud: THREE.Points | null;
  } | null = null;

  /** JR1-505: apply lighting control values in real time. */
  $: if (ambientLight) ambientLight.intensity = ambientIntensity;
  $: if (directionalLight) directionalLight.intensity = directionalIntensity;

  /** JR1-504: Create synthetic point cloud for performance testing. */
  function createSyntheticPointCloud(vertexCount: number): THREE.Points {
    const positions = new Float32Array(vertexCount * 3);
    const size = 200;
    for (let i = 0; i < vertexCount * 3; i += 3) {
      positions[i] = (Math.random() - 0.5) * size;
      positions[i + 1] = (Math.random() - 0.5) * size;
      positions[i + 2] = (Math.random() - 0.5) * size;
    }
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    geometry.computeBoundingSphere();
    const material = new THREE.PointsMaterial({
      size: 0.5,
      color: 0x4a90d9,
      sizeAttenuation: true,
    });
    return new THREE.Points(geometry, material);
  }

  /** JR1-504: Start performance test (100K, 500K, 1M vertices). */
  function runPerformanceTest() {
    if (perfTest || !scene || !controls) return;
    const sizes = [100_000, 500_000, 1_000_000];
    const saved = pointCloud;
    if (pointCloud) {
      scene.remove(pointCloud);
      pointCloud.geometry.dispose();
      (pointCloud.material as THREE.Material).dispose();
    }
    pointCloud = createSyntheticPointCloud(sizes[0]);
    scene.add(pointCloud);
    const box = new THREE.Box3().setFromObject(pointCloud);
    const center = box.getCenter(new THREE.Vector3());
    controls.target.copy(center);
    meshBox = null;
    meshStats = null;
    perfResult = null;
    perfTest = {
      index: 0,
      sizes,
      startTime: performance.now(),
      frameCount: 0,
      results: {},
      savedPointCloud: saved,
    };
  }

  /** JR1-504: Label for current perf test size. */
  function perfSizeLabel(n: number): string {
    if (n >= 1_000_000) return "1M";
    if (n >= 1_000) return n / 1_000 + "K";
    return String(n);
  }

  /**
   * Sprint A stub: live point cloud loading from the backend has been retired alongside the
   * 2.5D mesh pipeline. Sprint B will introduce `generate_point_cloud` (ADR-012) and Sprint C
   * will re-wire this button to render the resulting 3D surface point cloud inside the blank
   * envelope wireframe.
   */
  async function loadMesh() {
    meshLoading = true;
    meshError = "";
    try {
      if (pointCloud) {
        scene.remove(pointCloud);
        pointCloud.geometry.dispose();
        (pointCloud.material as THREE.Material).dispose();
        pointCloud = null;
      }
      meshBox = null;
      meshStats = null;
      meshError =
        "3D point cloud preview will be wired in Sprint B (generate_point_cloud, ADR-012).";
    } finally {
      meshLoading = false;
    }
  }

  /**
   * JR1-501: Set camera to preset view (top, front, side). Frames mesh when meshBox is set.
   */
  function setCameraPreset(preset: "top" | "front" | "side") {
    const box = meshBox;
    if (!box || !camera || !controls) return;
    const center = box.getCenter(new THREE.Vector3());
    const size = box.getSize(new THREE.Vector3());
    const maxDim = Math.max(size.x, size.y, size.z, 1);
    const distance = maxDim * 1.2;
    controls.target.copy(center);
    switch (preset) {
      case "top":
        camera.position.set(center.x, center.y + distance, center.z);
        break;
      case "front":
        camera.position.set(center.x, center.y, center.z + distance);
        break;
      case "side":
        camera.position.set(center.x + distance, center.y, center.z);
        break;
    }
  }

  function onResize(entry?: ResizeObserverEntry) {
    const width = entry?.contentRect.width ?? container.clientWidth;
    const height = entry?.contentRect.height ?? container.clientHeight;
    if (!camera || !renderer || width <= 0 || height <= 0) return;
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  }

  function animate() {
    animationId = requestAnimationFrame(animate);
    // JR1-504: advance perf test every 3s
    if (perfTest && scene && pointCloud) {
      perfTest.frameCount++;
      const elapsed = (performance.now() - perfTest.startTime) / 1000;
      if (elapsed >= 3) {
        const fps = Math.round(perfTest.frameCount / elapsed);
        const n = perfTest.sizes[perfTest.index];
        perfTest.results[perfSizeLabel(n)] = fps;
        perfTest.index++;
        if (perfTest.index >= perfTest.sizes.length) {
          perfResult = { "100K": perfTest.results["100K"] ?? 0, "500K": perfTest.results["500K"] ?? 0, "1M": perfTest.results["1M"] ?? 0 };
          scene.remove(pointCloud!);
          pointCloud!.geometry.dispose();
          (pointCloud!.material as THREE.Material).dispose();
          pointCloud = perfTest.savedPointCloud;
          if (pointCloud) scene.add(pointCloud);
          perfTest = null;
        } else {
          scene.remove(pointCloud);
          pointCloud.geometry.dispose();
          (pointCloud.material as THREE.Material).dispose();
          const nextN = perfTest.sizes[perfTest.index];
          pointCloud = createSyntheticPointCloud(nextN);
          scene.add(pointCloud);
          perfTest.startTime = performance.now();
          perfTest.frameCount = 0;
        }
      }
    }
    controls?.update();
    renderer?.render(scene, camera);
  }

  onMount(() => {
    if (!container) return;

    // UI-501, UI-502: Scene, camera, renderer, lights, grid
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1e293b);

    const width = container.clientWidth || 1;
    const height = container.clientHeight || 300;
    camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 10000);
    camera.position.set(200, 150, 200);

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    container.appendChild(renderer.domElement);

    // JR1-505: lights (intensity controlled by sliders; 1 unit = 1 mm per JR1-502)
    ambientLight = new THREE.AmbientLight(0xffffff, ambientIntensity);
    scene.add(ambientLight);
    directionalLight = new THREE.DirectionalLight(0xffffff, directionalIntensity);
    directionalLight.position.set(100, 200, 100);
    scene.add(directionalLight);

    // JR1-502: Grid floor — 400 mm × 400 mm, 20 divisions (20 mm spacing). Scale matches mesh (mm).
    gridHelper = new THREE.GridHelper(400, 20, 0x475569, 0x334155);
    scene.add(gridHelper);

    // UI-505: Orbit controls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.screenSpacePanning = true;
    controls.minDistance = 10;
    controls.maxDistance = 2000;

    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) onResize(entry);
    });
    resizeObserver.observe(container);

    animate();
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
    resizeObserver = null;
    if (animationId != null) cancelAnimationFrame(animationId);
    if (pointCloud) {
      scene?.remove(pointCloud);
      pointCloud.geometry.dispose();
      (pointCloud.material as THREE.Material).dispose();
    }
    controls?.dispose();
    renderer?.dispose();
    if (container?.contains(renderer?.domElement)) container.removeChild(renderer.domElement);
  });
</script>

<div
  class="preview3d flex flex-col w-full h-full min-h-[300px] rounded overflow-hidden bg-slate-800"
  data-target-width-mm={targetWidthMm ?? ""}
  data-target-height-mm={targetHeightMm ?? ""}
>
  <!-- Canvas container: Three.js renderer appended here -->
  <div
    class="preview3d-canvas flex-1 min-h-[200px] relative flex items-center justify-center"
    bind:this={container}
    role="img"
    aria-label="3D point cloud preview"
  ></div>

  <!--
    Sprint A toolbar: 2.5D mesh "Load mesh" + Wireframe/Solid render-mode buttons have been
    retired. Camera presets, lighting sliders, and the perf-test button remain so the scene
    infrastructure is exercised end-to-end. Sprint B will replace "Load mesh" with a
    "Generate point cloud" action wired to ADR-012.
  -->
  <div
    class="flex items-center gap-3 px-3 py-2 border-t border-slate-600 bg-slate-900 text-slate-200 text-sm"
  >
    <button
      type="button"
      class="px-3 py-1.5 rounded bg-slate-600 hover:bg-slate-500 focus:outline-none focus:ring-2 focus:ring-slate-400 disabled:opacity-50"
      disabled={meshLoading}
      on:click={loadMesh}
      title="Sprint A stub — point cloud generation is wired in Sprint B"
    >
      {meshLoading ? "Loading…" : "Load point cloud (Sprint B)"}
    </button>
    <!-- JR1-501: Camera presets (top, front, side) -->
    <span class="text-slate-500">View:</span>
    <div class="flex gap-1" role="group" aria-label="Camera presets">
      <button
        type="button"
        class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 disabled:opacity-50 disabled:cursor-not-allowed"
        title="Top view (look down)"
        disabled={!meshBox}
        on:click={() => setCameraPreset("top")}
      >
        Top
      </button>
      <button
        type="button"
        class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 disabled:opacity-50 disabled:cursor-not-allowed"
        title="Front view"
        disabled={!meshBox}
        on:click={() => setCameraPreset("front")}
      >
        Front
      </button>
      <button
        type="button"
        class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 disabled:opacity-50 disabled:cursor-not-allowed"
        title="Side view"
        disabled={!meshBox}
        on:click={() => setCameraPreset("side")}
      >
        Side
      </button>
    </div>
    <!-- JR1-505: Lighting controls -->
    <span class="text-slate-500">Light:</span>
    <label class="flex items-center gap-1 text-xs">
      <span class="text-slate-400 w-12">Amb.</span>
      <input
        type="range"
        min="0"
        max="2"
        step="0.1"
        bind:value={ambientIntensity}
        class="w-16 accent-slate-500"
        title="Ambient light intensity"
      />
    </label>
    <label class="flex items-center gap-1 text-xs">
      <span class="text-slate-400 w-12">Dir.</span>
      <input
        type="range"
        min="0"
        max="2"
        step="0.1"
        bind:value={directionalIntensity}
        class="w-16 accent-slate-500"
        title="Directional light intensity"
      />
    </label>
    <!-- JR1-504: Performance test -->
    <button
      type="button"
      class="px-2 py-1 rounded bg-slate-700 hover:bg-slate-600 focus:outline-none focus:ring-2 focus:ring-slate-400 disabled:opacity-50 disabled:cursor-not-allowed text-xs"
      title="Run performance test (100K, 500K, 1M vertices, ~9s)"
      disabled={!!perfTest}
      on:click={runPerformanceTest}
    >
      {perfTest ? `Perf… ${perfSizeLabel(perfTest.sizes[perfTest.index])}` : "Perf test"}
    </button>
    {#if perfResult}
      <span class="text-slate-400 text-xs" role="status" title="FPS for 100K, 500K, 1M vertices">
        100K: {perfResult["100K"]} | 500K: {perfResult["500K"]} | 1M: {perfResult["1M"]} FPS
      </span>
    {/if}
    <!-- JR1-503: Mesh statistics (vertex count, bounds in mm) -->
    {#if meshStats}
      <span class="text-slate-400 text-xs ml-auto" role="status">
        <span title="Vertex count">Vertices: {meshStats.vertexCount.toLocaleString()}</span>
        <span class="text-slate-500 mx-1">·</span>
        <span title="Bounds in mm">Bounds (mm): X {meshStats.minX.toFixed(1)}–{meshStats.maxX.toFixed(1)}, Y {meshStats.minY.toFixed(1)}–{meshStats.maxY.toFixed(1)}, Z {meshStats.minZ.toFixed(1)}–{meshStats.maxZ.toFixed(1)}</span>
      </span>
    {/if}
    {#if meshError}
      <p class="text-amber-400 flex-1 truncate" role="alert" title={meshError}>
        {meshError}
      </p>
    {/if}
  </div>
</div>

<style>
  .preview3d-canvas {
    outline: none;
  }
  /* Canvas is appended by Three.js at runtime */
  .preview3d-canvas :global(canvas) {
    display: block;
    width: 100%;
    height: 100%;
  }
</style>
