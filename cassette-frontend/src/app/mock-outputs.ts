import { Output } from './output';

export const OUTPUTS: Output[] = [
  { id: 1, name: 'Frame', type: "matrix", active: true },
  { id: 2, name: 'Front Wheel', type: "matrix", active: false },
  { id: 3, name: 'Rear Wheel', type: "matrix", active: false },
  { id: 4, name: 'LED Strip A', type: "strip", active: true },
  { id: 5, name: 'LED Strip B', type: "strip", active: false },
];