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
    option_type: EffectOptionType;
}

export interface AnalogOption extends EffectOption {
    min: number;
    max: number;
    analog_value: number;
}

export interface BooleanOption extends EffectOption {
    boolean_value: boolean;
}

export interface ColorOption extends EffectOption {
    color_value: string;
}

export interface SelectOption extends EffectOption {
    options: string[];
    select_value: string;
}