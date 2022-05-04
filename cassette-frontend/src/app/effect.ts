import { EffectOption } from './effect-option';


export interface Effect {
    id: number;
    name: string;
    options: EffectOption[];
}

export interface EffectsWrapper {
    activeId: number;
    effects: Effect[];
}