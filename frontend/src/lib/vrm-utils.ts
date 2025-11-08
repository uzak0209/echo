/**
 * VRMモデルのURLを生成するユーティリティ関数
 */

// VRoid Hubのサンプルモデル（パブリックドメイン）
const SAMPLE_VRM_MODELS = [
  // Three.js VRMの公式サンプルモデル（GitHub上）
  'https://raw.githubusercontent.com/pixiv/three-vrm/dev/packages/three-vrm/examples/models/VRM1_Constraint_Twist_Sample.vrm',
  // VRoid公式のサンプルモデル
  'https://cdn.jsdelivr.net/gh/pixiv/three-vrm@dev/packages/three-vrm/examples/models/VRM1_Constraint_Twist_Sample.vrm',
  // 別のVRMサンプル（あれば）
  'https://raw.githubusercontent.com/vrm-c/vrm-specification/master/samples/Alicia/AliciaSolid.vrm',
  // 追加のサンプル
  'https://raw.githubusercontent.com/vrm-c/vrm-specification/master/samples/AvatarSample_A.vrm',
  'https://raw.githubusercontent.com/vrm-c/vrm-specification/master/samples/AvatarSample_B.vrm',
];

/**
 * ユーザーIDに基づいてVRMモデルのURLを取得
 * @param userId ユーザーID
 * @returns VRMモデルのURL
 */
export function getVRMModelUrl(userId: string): string {
  // ユーザーIDをハッシュ化して、モデルを決定的に選択
  const hash = simpleHash(userId);
  const index = hash % SAMPLE_VRM_MODELS.length;
  return SAMPLE_VRM_MODELS[index];
}

/**
 * ランダムなVRMモデルのURLを取得
 * @returns VRMモデルのURL
 */
export function getRandomVRMModelUrl(): string {
  const index = Math.floor(Math.random() * SAMPLE_VRM_MODELS.length);
  return SAMPLE_VRM_MODELS[index];
}

/**
 * 簡単なハッシュ関数（文字列を数値に変換）
 */
function simpleHash(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash);
}

/**
 * リアクションタイプをVRM表情名にマッピング
 */
export function mapReactionToExpression(reaction: string | null): string | null {
  if (!reaction) return null;

  const reactionMap: Record<string, string> = {
    'laugh': 'happy',
    'empathy': 'relaxed',
    'surprise': 'surprised',
    'sad': 'sad',
    'confused': 'angry',
  };

  return reactionMap[reaction.toLowerCase()] || null;
}
