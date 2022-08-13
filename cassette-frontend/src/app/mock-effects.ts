import { Effect, EffectsWrapper } from './effect';
import { EffectOptionType } from './effect-option';
import { AnalogOption } from './effect-option';
import { BooleanOption } from './effect-option';
import { ColorOption } from './effect-option';
import { SelectOption } from './effect-option';

export const EFFECTS: EffectsWrapper = 
{
    activeId: 1,
    effects: [
        <Effect>{
            id: 1,
            name: 'Image Effect 1',
            options: [
                <BooleanOption>{
                    id: 1,
                    name: 'Advance GIF on beat',
                    type: EffectOptionType.Boolean,
                    value: false
                },
                <SelectOption>{
                    id: 2,
                    name: 'Image',
                    type: EffectOptionType.Select,
                    options: ['https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fbucketeer-e05bbc84-baa3-437e-9518-adb32be77984.s3.amazonaws.com%2Fpublic%2Fimages%2F05b87bec-b910-4de7-b93a-f65327c20da9_960x648.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png'],
                    value: 'https://ice45.link/utZQ.png'
                },
                <AnalogOption>{
                    id: 3,
                    name: 'Position X',
                    type: EffectOptionType.Analog,
                    value: 0,
                    min: 1,
                    max: 30,
                    step: 1
                },
                <AnalogOption>{
                    id: 4,
                    name: 'Position Y',
                    type: EffectOptionType.Analog,
                    value: 0,
                    min: 1,
                    max: 30,
                    step: 1
                },
                <AnalogOption>{
                    id: 5,
                    name: 'Scale',
                    type: EffectOptionType.Analog,
                    value: 1,
                    min: 0.1,
                    max: 5,
                    step: 0.1
                },

    
            ]
        }
        ,
    <Effect>{
        id: 2,
        name: 'Effect 1',
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
    <Effect>{
        id: 3,
        name: 'Effect 2',
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
]
};


