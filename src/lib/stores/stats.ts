import { writable, type Writable } from 'svelte/store';

/** Base stat keys */
export const baseKeys = [
  'Strength',
  'Speed',
  'Toughness',
  'Reflexes',
  'Hearing',
  'Observation',
  'Ancient Languages',
  'Combat Technique',
  'Premonition',
  'Bluff',
  'Magical sense',
  'Aura hardening'
] as const;
export type StatKey = typeof baseKeys[number];

/** Extra top-level metrics */
export type ExtraKey = 'magicalPower' | 'magicalKnowledge' | 'availablePoints';

/** Union of all metric keys */
export type AllKeys = StatKey | ExtraKey;

/** Central stats shape */
export type Stats = Record<AllKeys, number>;

export interface StatDelta {
  key: AllKeys;
  oldValue: number;
  newValue: number;
  timestamp: Date;
}

export interface DeltasStore {
  subscribe: Writable<StatDelta[]>['subscribe'];
  clear: () => void;
}

export interface StatsStore extends Writable<Stats> {
  deltas: DeltasStore;
  addStat: (key: AllKeys, amount: number) => void;
  resetDeltas: () => void;
}

function createStatsStore(): StatsStore {
  // Initialize all stats + extras
  const initial: Stats = {
    // Base stats
    Strength: 0,
    Speed: 0,
    Toughness: 0,
    Reflexes: 0,
    Hearing: 0,
    Observation: 0,
    'Ancient Languages': 0,
    'Combat Technique': 0,
    Premonition: 0,
    Bluff: 0,
    'Magical sense': 0,
    'Aura hardening': 0,

    // Special metrics
    magicalPower: 1,
    magicalKnowledge: 1,

    // Pool of points
    availablePoints: 4,
  };

  const stats = writable<Stats>(initial);
  const deltasInternal = writable<StatDelta[]>([]);

  const deltas: DeltasStore = {
    subscribe: deltasInternal.subscribe,
    clear: () => deltasInternal.set([]),
  };

  function addStat(key: AllKeys, amount: number) {
    stats.update(curr => {
      let delta = amount;
      const oldValue = curr[key];

      // If updating a base stat, enforce max 4 and availablePoints
      if ((baseKeys as readonly string[]).includes(key)) {
        const maxIncrease = Math.min(
          delta,
          curr.availablePoints,
          4 - oldValue
        );
        if (maxIncrease <= 0) {
          console.warn(`Cannot increase ${key} beyond limits`);
          return curr;
        }
        delta = maxIncrease;
      }

      const newValue = oldValue + delta;

      // Log the change
      deltasInternal.update(log => [
        ...log,
        { key, oldValue, newValue, timestamp: new Date() }
      ]);

      // Build next state
      const next: Stats = {
        ...curr,
        [key]: newValue,
        // Deduct pool if base stat
        ...((baseKeys as readonly string[]).includes(key) && {
          availablePoints: curr.availablePoints - delta
        })
      };

      return next;
    });
  }

  function resetDeltas() {
    deltas.clear();
  }

  return {
    subscribe: stats.subscribe,
    set: stats.set,
    update: stats.update,
    deltas,
    addStat,
    resetDeltas
  };
}

export const statsStore = createStatsStore();
