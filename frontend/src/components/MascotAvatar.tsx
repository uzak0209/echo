'use client';

import { useEffect, useRef, useState } from 'react';

interface MascotAvatarProps {
  userId: string;
  expression?: string | null; // 'laugh', 'empathy', 'surprise', 'sad', 'confused'
}

export function MascotAvatar({ userId, expression }: MascotAvatarProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isLoading, setIsLoading] = useState(true);
  const mascotRef = useRef<any>({});
  const sceneRef = useRef<any>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    let mounted = true;
    let scene: any = null;
    let camera: any = null;
    let renderer: any = null;
    let controls: any = null;
    let animationId: number;

    const initThreeScene = async () => {
      try {
        // Dynamic imports
        const THREE = await import('three');
        const { OrbitControls } = await import('three/examples/jsm/controls/OrbitControls.js');

        if (!mounted || !canvasRef.current) return;

        // Setup scene
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf5f5f5);

        // Setup camera
        camera = new THREE.PerspectiveCamera(
          45,
          canvasRef.current.clientWidth / canvasRef.current.clientHeight,
          0.1,
          1000
        );
        camera.position.set(0, 0, 5);

        // Setup renderer
        renderer = new THREE.WebGLRenderer({
          canvas: canvasRef.current,
          alpha: true,
          antialias: true,
        });
        renderer.setSize(canvasRef.current.clientWidth, canvasRef.current.clientHeight);
        renderer.setPixelRatio(window.devicePixelRatio);

        // Setup lights
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
        scene.add(ambientLight);

        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
        directionalLight.position.set(2, 3, 4);
        scene.add(directionalLight);

        // Setup orbit controls
        controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;
        controls.enablePan = false;
        controls.enableZoom = true;
        controls.minDistance = 3;
        controls.maxDistance = 8;

        // Generate color from userId
        const color = generateColorFromId(userId);

        // Create mascot character
        const mascotGroup = new THREE.Group();

        // Body (sphere)
        const bodyGeometry = new THREE.SphereGeometry(1, 32, 32);
        const bodyMaterial = new THREE.MeshPhongMaterial({
          color: color,
          shininess: 30,
        });
        const body = new THREE.Mesh(bodyGeometry, bodyMaterial);
        mascotGroup.add(body);
        mascotRef.current.body = body;
        mascotRef.current.originalColor = color; // Save original color
        mascotRef.current.effectElements = [];

        // Eyes (two spheres)
        const eyeGeometry = new THREE.SphereGeometry(0.15, 16, 16);
        const eyeMaterial = new THREE.MeshPhongMaterial({
          color: 0x000000,
        });

        const leftEye = new THREE.Mesh(eyeGeometry, eyeMaterial);
        leftEye.position.set(-0.3, 0.3, 0.85);
        mascotGroup.add(leftEye);
        mascotRef.current.leftEye = leftEye;

        const rightEye = new THREE.Mesh(eyeGeometry, eyeMaterial);
        rightEye.position.set(0.3, 0.3, 0.85);
        mascotGroup.add(rightEye);
        mascotRef.current.rightEye = rightEye;

        // Mouth (will be updated based on expression)
        const mouthCurve = new THREE.EllipseCurve(
          0, 0,
          0.4, 0.2,
          0, Math.PI,
          false,
          0
        );
        const mouthPoints = mouthCurve.getPoints(50);
        const mouthGeometry = new THREE.BufferGeometry().setFromPoints(mouthPoints);
        const mouthMaterial = new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 3 });
        const mouth = new THREE.Line(mouthGeometry, mouthMaterial);
        mouth.position.set(0, -0.2, 0.85);
        mascotGroup.add(mouth);
        mascotRef.current.mouth = mouth;
        mascotRef.current.mouthMaterial = mouthMaterial;

        // Cheeks (two small spheres)
        const cheekGeometry = new THREE.SphereGeometry(0.1, 16, 16);
        const cheekMaterial = new THREE.MeshPhongMaterial({
          color: 0xff9999,
          transparent: true,
          opacity: 0.6,
        });

        const leftCheek = new THREE.Mesh(cheekGeometry, cheekMaterial);
        leftCheek.position.set(-0.6, 0, 0.7);
        mascotGroup.add(leftCheek);
        mascotRef.current.leftCheek = leftCheek;

        const rightCheek = new THREE.Mesh(cheekGeometry, cheekMaterial);
        rightCheek.position.set(0.6, 0, 0.7);
        mascotGroup.add(rightCheek);
        mascotRef.current.rightCheek = rightCheek;

        scene.add(mascotGroup);
        mascotRef.current.group = mascotGroup;
        sceneRef.current = scene;

        setIsLoading(false);

        // Animation loop
        const clock = new THREE.Clock();
        let floatOffset = 0;

        const animate = () => {
          animationId = requestAnimationFrame(animate);

          floatOffset += clock.getDelta();

          // Gentle floating animation (only up and down, no rotation)
          if (mascotRef.current.group) {
            mascotRef.current.group.position.y = Math.sin(floatOffset * 2) * 0.1;
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
        setIsLoading(false);
      }
    };

    // Generate color from userId (deterministic)
    const generateColorFromId = (id: string): number => {
      let hash = 0;
      for (let i = 0; i < id.length; i++) {
        hash = id.charCodeAt(i) + ((hash << 5) - hash);
      }

      // Generate pastel colors
      const hue = Math.abs(hash % 360);
      const saturation = 70;
      const lightness = 75;

      return hslToRgbHex(hue, saturation, lightness);
    };

    const hslToRgbHex = (h: number, s: number, l: number): number => {
      s /= 100;
      l /= 100;

      const c = (1 - Math.abs(2 * l - 1)) * s;
      const x = c * (1 - Math.abs((h / 60) % 2 - 1));
      const m = l - c / 2;
      let r = 0, g = 0, b = 0;

      if (0 <= h && h < 60) {
        r = c; g = x; b = 0;
      } else if (60 <= h && h < 120) {
        r = x; g = c; b = 0;
      } else if (120 <= h && h < 180) {
        r = 0; g = c; b = x;
      } else if (180 <= h && h < 240) {
        r = 0; g = x; b = c;
      } else if (240 <= h && h < 300) {
        r = x; g = 0; b = c;
      } else if (300 <= h && h < 360) {
        r = c; g = 0; b = x;
      }

      const rHex = Math.round((r + m) * 255);
      const gHex = Math.round((g + m) * 255);
      const bHex = Math.round((b + m) * 255);

      return (rHex << 16) | (gHex << 8) | bHex;
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
    };
  }, [userId]);

  // Update expression when it changes
  useEffect(() => {
    const updateExpression = async () => {
      const THREE = await import('three');

      if (!mascotRef.current.mouth || !sceneRef.current) return;

      // Remove old mouth and effects
      const group = mascotRef.current.group;
      if (group && mascotRef.current.mouth) {
        group.remove(mascotRef.current.mouth);
      }

      // Remove old effect elements
      if (mascotRef.current.effectElements) {
        mascotRef.current.effectElements.forEach((elem: any) => {
          group.remove(elem);
        });
      }
      mascotRef.current.effectElements = [];

      let mouthCurve;
      let eyeScaleX = 1.0;
      let eyeScaleY = 1.0;
      let cheekOpacity = 0.6;
      let bodyTilt = 0;
      let bodyColor = null;

      // Create different expressions based on reaction
      switch (expression?.toLowerCase()) {
        case 'laugh': { // 笑顔 - 超ハッピー！
          // 大きな笑顔
          mouthCurve = new THREE.EllipseCurve(0, 0, 0.6, 0.35, 0, Math.PI, false, 0);
          // 目を線に（^_^）
          eyeScaleX = 1.5;
          eyeScaleY = 0.1;
          cheekOpacity = 1.0;

          // 体を少し明るく
          bodyColor = 0xffffcc;

          // キラキラエフェクト追加
          for (let i = 0; i < 5; i++) {
            const starGeometry = new THREE.SphereGeometry(0.05, 8, 8);
            const starMaterial = new THREE.MeshPhongMaterial({
              color: 0xffff00,
              emissive: 0xffff00,
              transparent: true,
              opacity: 0.8,
            });
            const star = new THREE.Mesh(starGeometry, starMaterial);
            const angle = (i / 5) * Math.PI * 2;
            star.position.set(
              Math.cos(angle) * 1.3,
              Math.sin(angle) * 1.3 + 0.3,
              0.3
            );
            group.add(star);
            mascotRef.current.effectElements.push(star);
          }
          break;
        }

        case 'empathy': { // 共感 - ハート付き
          mouthCurve = new THREE.EllipseCurve(0, 0, 0.45, 0.2, 0, Math.PI, false, 0);
          eyeScaleX = 1.0;
          eyeScaleY = 0.8;
          cheekOpacity = 0.8;

          // ピンク色に
          bodyColor = 0xffccdd;

          // ハート追加
          const heartShape = new THREE.Shape();
          heartShape.moveTo(0, 0);
          heartShape.bezierCurveTo(0, -0.3, -0.5, -0.3, -0.5, 0);
          heartShape.bezierCurveTo(-0.5, 0.3, 0, 0.5, 0, 1);
          heartShape.bezierCurveTo(0, 0.5, 0.5, 0.3, 0.5, 0);
          heartShape.bezierCurveTo(0.5, -0.3, 0, -0.3, 0, 0);

          const heartGeometry = new THREE.ShapeGeometry(heartShape);
          const heartMaterial = new THREE.MeshPhongMaterial({
            color: 0xff69b4,
            transparent: true,
            opacity: 0.7,
            side: THREE.DoubleSide,
          });
          const heart = new THREE.Mesh(heartGeometry, heartMaterial);
          heart.position.set(0.8, 0.8, 0.5);
          heart.scale.set(0.2, 0.2, 0.2);
          group.add(heart);
          mascotRef.current.effectElements.push(heart);
          break;
        }

        case 'surprise': { // 驚き - 目と口が超大きい
          // 大きく開いた口
          mouthCurve = new THREE.EllipseCurve(0, 0, 0.35, 0.45, 0, Math.PI * 2, false, 0);
          // 超大きい目
          eyeScaleX = 2.0;
          eyeScaleY = 2.0;
          cheekOpacity = 0.2;

          // ビックリマーク追加
          const exclamationGeometry = new THREE.CylinderGeometry(0.08, 0.08, 0.4, 8);
          const exclamationMaterial = new THREE.MeshPhongMaterial({ color: 0xff0000 });
          const exclamation = new THREE.Mesh(exclamationGeometry, exclamationMaterial);
          exclamation.position.set(0.7, 1.0, 0.5);
          group.add(exclamation);

          const dotGeometry = new THREE.SphereGeometry(0.1, 8, 8);
          const dot = new THREE.Mesh(dotGeometry, exclamationMaterial);
          dot.position.set(0.7, 0.6, 0.5);
          group.add(dot);

          mascotRef.current.effectElements.push(exclamation, dot);
          break;
        }

        case 'sad': { // 悲しい - 涙付き
          // 大きく下がった口
          mouthCurve = new THREE.EllipseCurve(0, 0, 0.5, 0.3, Math.PI, Math.PI * 2, false, 0);
          eyeScaleX = 0.7;
          eyeScaleY = 1.0;
          cheekOpacity = 0.1;

          // 体を暗く
          bodyColor = 0xaaaacc;

          // 涙を追加
          const tearGeometry = new THREE.SphereGeometry(0.1, 16, 16);
          const tearMaterial = new THREE.MeshPhongMaterial({
            color: 0x4444ff,
            transparent: true,
            opacity: 0.7,
          });

          const leftTear = new THREE.Mesh(tearGeometry, tearMaterial);
          leftTear.position.set(-0.3, 0.1, 0.9);
          leftTear.scale.set(0.8, 1.5, 0.8);
          group.add(leftTear);

          const rightTear = new THREE.Mesh(tearGeometry, tearMaterial);
          rightTear.position.set(0.3, 0.1, 0.9);
          rightTear.scale.set(0.8, 1.5, 0.8);
          group.add(rightTear);

          mascotRef.current.effectElements.push(leftTear, rightTear);
          break;
        }

        case 'confused': { // 困惑 - 汗と波線の口
          // 波打つ口
          const points = [];
          for (let i = 0; i <= 30; i++) {
            const t = i / 30;
            const x = (t - 0.5) * 1.0;
            const y = Math.sin(t * Math.PI * 4) * 0.15 - 0.1;
            points.push(new THREE.Vector2(x, y));
          }
          mouthCurve = new THREE.SplineCurve(points);

          eyeScaleX = 0.5;
          eyeScaleY = 0.5;
          cheekOpacity = 0.3;
          bodyTilt = 0.2; // 体を傾ける

          // 汗マーク追加
          const sweatGeometry = new THREE.SphereGeometry(0.12, 16, 16);
          const sweatMaterial = new THREE.MeshPhongMaterial({
            color: 0x88ccff,
            transparent: true,
            opacity: 0.7,
          });
          const sweat = new THREE.Mesh(sweatGeometry, sweatMaterial);
          sweat.position.set(-0.7, 0.7, 0.6);
          sweat.scale.set(0.8, 1.3, 0.8);
          group.add(sweat);

          mascotRef.current.effectElements.push(sweat);
          break;
        }

        default: { // デフォルト
          mouthCurve = new THREE.EllipseCurve(0, 0, 0.3, 0.1, 0, Math.PI, false, 0);
          eyeScaleX = 1.0;
          eyeScaleY = 1.0;
          cheekOpacity = 0.6;
        }
      }

      // Create new mouth
      const mouthPoints = mouthCurve.getPoints(50);
      const mouthGeometry = new THREE.BufferGeometry().setFromPoints(mouthPoints);
      const mouthMaterial = new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 4 });
      const mouth = new THREE.Line(mouthGeometry, mouthMaterial);
      mouth.position.set(0, -0.2, 0.85);

      if (group) {
        group.add(mouth);
      }

      mascotRef.current.mouth = mouth;

      // Update eyes scale
      if (mascotRef.current.leftEye && mascotRef.current.rightEye) {
        mascotRef.current.leftEye.scale.set(eyeScaleX, eyeScaleY, 1.0);
        mascotRef.current.rightEye.scale.set(eyeScaleX, eyeScaleY, 1.0);
      }

      // Update cheeks opacity
      if (mascotRef.current.leftCheek && mascotRef.current.rightCheek) {
        const leftCheekMat = mascotRef.current.leftCheek.material as any;
        const rightCheekMat = mascotRef.current.rightCheek.material as any;
        leftCheekMat.opacity = cheekOpacity;
        rightCheekMat.opacity = cheekOpacity;

        // Make cheeks bigger for happy expressions
        if (expression?.toLowerCase() === 'laugh') {
          mascotRef.current.leftCheek.scale.set(1.5, 1.5, 1.5);
          mascotRef.current.rightCheek.scale.set(1.5, 1.5, 1.5);
        } else {
          mascotRef.current.leftCheek.scale.set(1.0, 1.0, 1.0);
          mascotRef.current.rightCheek.scale.set(1.0, 1.0, 1.0);
        }
      }

      // Update body color
      if (bodyColor && mascotRef.current.body) {
        (mascotRef.current.body.material as any).color.setHex(bodyColor);
      } else if (mascotRef.current.body && mascotRef.current.originalColor) {
        (mascotRef.current.body.material as any).color.setHex(mascotRef.current.originalColor);
      }

      // Update body tilt
      if (mascotRef.current.group) {
        mascotRef.current.group.rotation.z = bodyTilt;
      }
    };

    if (expression && mascotRef.current.group) {
      updateExpression();
    }
  }, [expression]);

  return (
    <div className="w-full h-full relative">
      {isLoading && (
        <div className="absolute inset-0 flex items-center justify-center bg-white/80 z-10">
          <p className="text-muted-foreground">Creating your mascot...</p>
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
