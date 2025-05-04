import { writable, type Writable } from 'svelte/store';

export type StatKey =
  | 'Strength'
  | 'Speed'
  | 'Toughness'
  | 'Reflexes'
  | 'Hearing'
  | 'Observation'
  | 'Ancient Languages'
  | 'Combat Technique'
  | 'Premonition'
  | 'Bluff'
  | 'Magical sense'
  | 'Aura hardening';

export type Stats = Record<StatKey, number>;

export interface StatDelta {
  key: StatKey;
  oldValue: number;
  newValue: number;
  timestamp: Date;
}

export interface DeltasStore {
  subscribe: Writable<StatDelta[]>['subscribe'];
  clear: () => void;
}

export interface StatsStore extends Writable<Stats> {
  /** Stream of all applied deltas (immutable) */
  deltas: DeltasStore;

  /** Add (or subtract) an amount to a stat */
  addStat: (key: StatKey, amount: number) => void;

  /** Clear the delta log */
  resetDeltas: () => void;
}

function createStatsStore(): StatsStore {
  // 1) Initialize all stats to zero
  const initialStats: Stats = {
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
    'Aura hardening': 0
  };

  // 2) Core stores
  const stats = writable<Stats>(initialStats);
  const deltasInternal = writable<StatDelta[]>([]);

  // 3) Expose only subscribe + clear on deltas
  const deltas: DeltasStore = {
    subscribe: deltasInternal.subscribe,
    clear: () => deltasInternal.set([])
  };

  // 4) Updating both stats + log atomically
  function addStat(key: StatKey, amount: number) {
    stats.update(curr => {
      const oldValue = curr[key];
      const newValue = oldValue + amount;

      // log the change
      deltasInternal.update(log => [
        ...log,
        { key, oldValue, newValue, timestamp: new Date() }
      ]);

      // return updated stats
      return { ...curr, [key]: newValue };
    });
  }

  function resetDeltas() {
    deltas.clear();
  }

  return {
    // Writable<Stats> interface
    subscribe: stats.subscribe,
    set: stats.set,
    update: stats.update,

    // our additions
    deltas,
    addStat,
    resetDeltas
  };
}

/** singleton instance for your app */
export const statsStore = createStatsStore();
