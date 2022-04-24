import { Effect } from './effect';
import { EffectOptionType } from './effect-option';
import { AnalogOption } from './effect-option';
import { BooleanOption } from './effect-option';
import { ColorOption } from './effect-option';
import { SelectOption } from './effect-option';

export const EFFECTS: Effect[] = [
    {
        id: 1,
        name: 'Effect 1',
        active: false,
        options: [
            <AnalogOption>{
                id: 1,
                name: 'Option 1',
                type: EffectOptionType.Analog,
                min: 0,
                max: 100,
                value: 50
            },
            <BooleanOption>{
                id: 2,
                name: 'Option 2',
                type: EffectOptionType.Boolean,
                value: true
            },
            <ColorOption>{
                id: 3,
                name: 'Option 3',
                type: EffectOptionType.Color,
                value: '#ff0000'
            },
            <SelectOption>{
                id: 4,
                name: 'Option 4',
                type: EffectOptionType.Select,
                options: ['Option 1', 'Option 2', 'Option 3'],
                value: 'Option 1'
            }
        ]
    },
    {
        id: 2,
        name: 'Effect 2',
        active: true,
        options: [
            <BooleanOption>{
                id: 1,
                name: 'Option 1',
                type: EffectOptionType.Boolean,
                value: false
            },
            <SelectOption>{
                id: 2,
                name: 'Option 2',
                type: EffectOptionType.Select,
                options: ['Option 19', 'Option 20', 'Option 32'],
                value: 'Option 20'
            },
            <AnalogOption>{
                id: 3,
                name: 'Option 3',
                type: EffectOptionType.Analog,
                min: 10,
                max: 200,
                value: 25
            },
            <ColorOption>{
                id: 4,
                name: 'Option 4',
                type: EffectOptionType.Color,
                value: '#bada55'
            },
        ]
    },
];


