'use client';

import { useEffect, useRef, useState } from 'react';

interface VRMAvatarProps {
  modelUrl: string;
  expression?: string | null;
}

export function VRMAvatar({ modelUrl, expression }: VRMAvatarProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    let mounted = true;
    let scene: any = null;
    let camera: any = null;
    let renderer: any = null;
    let vrm: any = null;
    let controls: any = null;
    let animationId: number;

    const initThreeScene = async () => {
      try {
        // Dynamic imports
        const THREE = await import('three');
        const { GLTFLoader } = await import('three/examples/jsm/loaders/GLTFLoader.js');
        const { VRMLoaderPlugin, VRMUtils } = await import('@pixiv/three-vrm');
        const { OrbitControls } = await import('three/examples/jsm/controls/OrbitControls.js');

        if (!mounted || !canvasRef.current) return;

        // Setup scene
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf0f0f0);

        // Setup camera
        camera = new THREE.PerspectiveCamera(
          30,
          canvasRef.current.clientWidth / canvasRef.current.clientHeight,
          0.1,
          1000
        );
        camera.position.set(0, 1.3, 1.5);

        // Setup renderer
        renderer = new THREE.WebGLRenderer({
          canvas: canvasRef.current,
          alpha: true,
          antialias: true,
        });
        renderer.setSize(canvasRef.current.clientWidth, canvasRef.current.clientHeight);
        renderer.setPixelRatio(window.devicePixelRatio);

        // Setup lights
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.7);
        scene.add(ambientLight);

        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
        directionalLight.position.set(1, 1, 1);
        scene.add(directionalLight);

        // Setup orbit controls
        controls = new OrbitControls(camera, renderer.domElement);
        controls.target.set(0, 1.3, 0);
        controls.enablePan = false;
        controls.enableZoom = true;
        controls.minDistance = 1;
        controls.maxDistance = 3;
        controls.update();

        // Load VRM
        const loader = new GLTFLoader();
        loader.register((parser: any) => new VRMLoaderPlugin(parser));

        loader.load(
          modelUrl,
          (gltf: any) => {
            if (!mounted) return;

            const loadedVrm = gltf.userData.vrm;
            if (loadedVrm) {
              VRMUtils.rotateVRM0(loadedVrm);
              scene.add(loadedVrm.scene);
              vrm = loadedVrm;
              setIsLoading(false);
            }
          },
          (progress: any) => {
            console.log('Loading VRM...', Math.floor(100 * progress.loaded / progress.total) + '%');
          },
          (error: any) => {
            console.error('Error loading VRM:', error);
            if (mounted) {
              setError('Failed to load 3D avatar');
              setIsLoading(false);
            }
          }
        );

        // Animation loop
        const clock = new THREE.Clock();
        const animate = () => {
          animationId = requestAnimationFrame(animate);

          if (vrm) {
            const delta = clock.getDelta();
            vrm.update(delta);
          }

          controls.update();
          renderer.render(scene, camera);
        };

        animate();

        // Handle resize
        const handleResize = () => {
          if (!canvasRef.current || !camera || !renderer) return;

          const width = canvasRef.current.clientWidth;
          const height = canvasRef.current.clientHeight;

          camera.aspect = width / height;
          camera.updateProjectionMatrix();
          renderer.setSize(width, height);
        };

        window.addEventListener('resize', handleResize);

        return () => {
          window.removeEventListener('resize', handleResize);
        };
      } catch (err) {
        console.error('Error initializing Three.js:', err);
        if (mounted) {
          setError('Failed to initialize 3D viewer');
          setIsLoading(false);
        }
      }
    };

    initThreeScene();

    return () => {
      mounted = false;
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
      if (renderer) {
        renderer.dispose();
      }
      if (vrm) {
        scene?.remove(vrm.scene);
      }
    };
  }, [modelUrl]);

  // Update expression
  useEffect(() => {
    // This will be implemented once VRM is loaded
    // For now, we'll skip expression changes
  }, [expression]);

  if (error) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <p className="text-destructive">{error}</p>
      </div>
    );
  }

  return (
    <div className="w-full h-full relative">
      {isLoading && (
        <div className="absolute inset-0 flex items-center justify-center bg-white/80 z-10">
          <p className="text-muted-foreground">Loading 3D Avatar...</p>
        </div>
      )}
      <canvas
        ref={canvasRef}
        className="w-full h-full"
        style={{ display: 'block' }}
      />
    </div>
  );
}
