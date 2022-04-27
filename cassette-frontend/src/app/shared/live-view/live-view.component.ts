import { Component, Inject, Input, OnInit } from '@angular/core';
import { ResizedEvent } from 'angular-resize-event';
import { Observable, of } from 'rxjs';
import { Pixel } from 'src/app/pixel';
import { BackendService } from 'src/app/services/backend.service';
import { DOCUMENT } from '@angular/common'; 

@Component({
  selector: 'live-view',
  templateUrl: './live-view.component.html',
  styleUrls: ['./live-view.component.css']
})
export class LiveViewComponent implements OnInit {
    @Input() n_horizontal: number = 0;
    @Input() n_vertical: number = 0;

    width: number = 0;
    gutter_size: number = 2;

    pixels: Observable<Pixel[]> = of([]);

    matrix_num: number = 1;


    constructor(@Inject(DOCUMENT) document: Document, private backendService: BackendService) { }

    ngOnInit(): void {
        // for pixel array initialization
        // for(let i=0; i<this.n_horizontal*this.n_vertical; i++) {
        //     // create new pixel interface object
        //     let pixel:Pixel = <Pixel>{
        //                                 r: 0,
        //                                 g: 0,
        //                                 b: 0,
        //                                 patched: false
        //                              };
        //     this.pixels.push(pixel);
        // }
        // this.backendService.getPixels().subscribe(pixels => {
        //     // set mat grid tiles to pixel colors
        //     // for pixel of pixels
        //     if(pixels.length == this.n_horizontal*this.n_vertical) {
        //         this.pixels = pixels;
        //     }
        // });
        // this.pixels = this.backendService.getPixels();
        this.backendService.getPixels().subscribe(pixels => {
            // set mat grid tiles to pixel colors
            // for pixel of pixels
            for (let i = 0; i < pixels.length; i++) {
                var pixel = document.getElementById("pixel_" + i);
                if(pixel != null) {
                    if(pixels[i].patched){
                        pixel.style.backgroundColor = this.getHex(pixels[i]);
                        pixel.classList.add("patched");
                        pixel.classList.remove("unpatched");
                    }
                    else
                    {
                        pixel.style.backgroundColor = "transparent";
                        pixel.classList.add("unpatched");
                        pixel.classList.remove("patched");
                    }
                }
            }
            // this.matrix_swap();
        });

    }

    onResized(event: ResizedEvent) {
        console.log("resized");
        if(event.newRect.height/this.n_vertical < event.newRect.width/this.n_horizontal) {
            this.width = this.n_horizontal * (event.newRect.height-this.gutter_size*(this.n_horizontal-1))/this.n_vertical;
        }else{
            this.width = this.n_horizontal * (event.newRect.width-this.gutter_size*(this.n_vertical-1))/this.n_horizontal;
        }
        console.log(this.width);
        console.log(this.pixels);
    }


    // public matrix_swap() {
    //     if(this.matrix_num == 1) {
    //         this.matrix_num = 2;
    //         // document.getElementById("matrix_2")!.style.visibility = "hidden";
    //         // document.getElementById("matrix_1")!.style.visibility = "visible";
    //     }else{
    //         this.matrix_num = 1;
    //         // document.getElementById("matrix_2")!.style.visibility = "visible";
    //         // document.getElementById("matrix_1")!.style.visibility = "hidden";
    //     }
    // }


    public setHex(hex: string, pixel: Pixel) {
                const rgb = this.hexToRgb(hex);
                if (rgb != null) {
                    pixel.r = rgb.r;
                    pixel.g = rgb.g;
                    pixel.b = rgb.b;
                }
            }
    
    private hexToRgb(hex: string) {
        const shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
        hex = hex.replace(shorthandRegex, (m, r, g, b) => {
            return r + r + g + g + b + b;
        });
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : null;
    }
        
        
        
    private componentToHex(c: number): string {
        var hex = c.toString(16);
        return hex.length == 1 ? "0" + hex : hex;
    }
    
    public getHex(pixel: Pixel): string {
        return "#" + this.componentToHex(pixel.r) + this.componentToHex(pixel.g) + this.componentToHex(pixel.b);
    }



}
