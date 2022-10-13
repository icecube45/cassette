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
                    option_type: EffectOptionType.Boolean,
                    boolean_value: false
                },
                <SelectOption>{
                    id: 2,
                    name: 'Image',
                    option_type: EffectOptionType.Select,
                    options: ['https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fbucketeer-e05bbc84-baa3-437e-9518-adb32be77984.s3.amazonaws.com%2Fpublic%2Fimages%2F05b87bec-b910-4de7-b93a-f65327c20da9_960x648.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png', 'https://ice45.link/9JPD.gif', 'https://ice45.link/0mcx.png', 'https://ice45.link/utZQ.png'],
                    select_value: 'https://ice45.link/utZQ.png'
                },
                <AnalogOption>{
                    id: 3,
                    name: 'Position X',
                    option_type: EffectOptionType.Analog,
                    analog_value: 0,
                    min: 1,
                    max: 30,
                    step: 1
                },
                <AnalogOption>{
                    id: 4,
                    name: 'Position Y',
                    option_type: EffectOptionType.Analog,
                    analog_value: 0,
                    min: 1,
                    max: 30,
                    step: 1
                },
                <AnalogOption>{
                    id: 5,
                    name: 'Scale',
                    option_type: EffectOptionType.Analog,
                    analog_value: 1,
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
                option_type: EffectOptionType.Analog,
                min: 0,
                max: 100,
                analog_value: 50
            },
            <BooleanOption>{
                id: 2,
                name: 'Option 2',
                option_type: EffectOptionType.Boolean,
                boolean_value: true
            },
            <ColorOption>{
                id: 3,
                name: 'Option 3',
                option_type: EffectOptionType.Color,
                color_value: '#ff0000'
            },
            <SelectOption>{
                id: 4,
                name: 'Option 4',
                option_type: EffectOptionType.Select,
                options: ['Option 1', 'Option 2', 'Option 3'],
                select_value: 'Option 1'
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
                option_type: EffectOptionType.Boolean,
                boolean_value: false
            },
            <SelectOption>{
                id: 2,
                name: 'Option 2',
                option_type: EffectOptionType.Select,
                options: ['Option 19', 'Option 20', 'Option 32'],
                select_value: 'Option 20'
            },
            <AnalogOption>{
                id: 3,
                name: 'Option 3',
                option_type: EffectOptionType.Analog,
                min: 10,
                max: 200,
                analog_value: 25
            },
            <ColorOption>{
                id: 4,
                name: 'Option 4',
                option_type: EffectOptionType.Color,
                color_value: '#bada55'
            },
        ]
    },
]
};


