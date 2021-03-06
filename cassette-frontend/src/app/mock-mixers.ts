import { Mixer } from './mixer';
import { EffectOptionType } from './effect-option';
import { AnalogOption } from './effect-option';
import { SelectOption } from './effect-option';

export const MIXER: Mixer = 
{
    id: 1,
    options: [
        <SelectOption>{
            id: 1,
            type: EffectOptionType.Select,
            options: ['Progressive',
                        'Linear',
                        'Left Shape',
                        'Right Shape',
                        'Left Intensity',
                        'Right Intensity',
                        'Left Overlay',
                        'Right Overlay',
                        'Left Overlay (Border)',
                        'Right Overlay (Border)'
                    ],
            value: 'Progressive'
        },
        <AnalogOption>{
            id: 2,
            name: 'Weight',
            type: EffectOptionType.Analog,
            min: 0,
            max: 100,
            value: 50
        }
    ]
};