import { EffectOption } from './effect-option';


export interface Effect {
    id: number;
    name: string;
    active: boolean;
    options: EffectOption[];
}