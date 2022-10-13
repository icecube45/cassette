import { Output } from './output';

export const OUTPUTS: Output[] = [
  { id: 1, name: 'Frame', output_type: "matrix", active: true },
  { id: 2, name: 'Front Wheel', output_type: "matrix", active: false },
  { id: 3, name: 'Rear Wheel', output_type: "matrix", active: false },
  { id: 4, name: 'LED Strip A', output_type: "strip", active: true },
  { id: 5, name: 'LED Strip B', output_type: "strip", active: false },
];