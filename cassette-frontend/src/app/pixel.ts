
// export class Pixel {
    
//     constructor(public r: number, public g: number, public b: number, public patched: boolean = true) {

//     }

    

//     public setHex(hex: string) {
//         const rgb = this.hexToRgb(hex);
//         if (rgb != null) {
//             this.r = rgb.r;
//             this.g = rgb.g;
//             this.b = rgb.b;
//         }
//     }
    
//     private hexToRgb(hex: string) {
//         const shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
//         hex = hex.replace(shorthandRegex, (m, r, g, b) => {
//           return r + r + g + g + b + b;
//         });
//         const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
//         return result ? {
//           r: parseInt(result[1], 16),
//           g: parseInt(result[2], 16),
//           b: parseInt(result[3], 16)
//         } : null;
//       }



//       private componentToHex(c: number): string {
//         var hex = c.toString(16);
//         return hex.length == 1 ? "0" + hex : hex;
//       }
      
//       public getHex(): string {
//         return "#" + this.componentToHex(this.r) + this.componentToHex(this.g) + this.componentToHex(this.b);
//       }
// }


export interface Pixel {
    r: number;
    g: number;
    b: number;
    patched: boolean;
}