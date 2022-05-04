export enum EffectOptionType {
    Analog = 'analog',
    Boolean = 'boolean',
    Color = 'color',
    Select = 'select',
    Unknown = 'unknown'
}

export interface EffectOption {
    id: number;
    name: string;
    type: EffectOptionType;
}

export interface AnalogOption extends EffectOption {
    min: number;
    max: number;
    value: number;
}

export interface BooleanOption extends EffectOption {
    value: boolean;
}

export interface ColorOption extends EffectOption {
    value: string;
}

export interface SelectOption extends EffectOption {
    options: string[];
    value: string;
}