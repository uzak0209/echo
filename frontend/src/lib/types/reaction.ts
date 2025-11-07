export enum ReactionType {
  SURPRISE = 'SURPRISE',
  EMPATHY = 'EMPATHY',
  LAUGH = 'LAUGH',
  SAD = 'SAD',
  CONFUSED = 'CONFUSED',
}

export const REACTION_EMOJIS: Record<ReactionType, string> = {
  [ReactionType.SURPRISE]: '😲',
  [ReactionType.EMPATHY]: '🥺',
  [ReactionType.LAUGH]: '😂',
  [ReactionType.SAD]: '😢',
  [ReactionType.CONFUSED]: '🤔',
};

export const REACTION_LABELS: Record<ReactionType, string> = {
  [ReactionType.SURPRISE]: '驚き',
  [ReactionType.EMPATHY]: '共感',
  [ReactionType.LAUGH]: '笑い',
  [ReactionType.SAD]: '悲しい',
  [ReactionType.CONFUSED]: '首を傾げる',
};
