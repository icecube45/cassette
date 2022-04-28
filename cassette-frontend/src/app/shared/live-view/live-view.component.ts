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

    gutter_size: number = 2;

    matrix_num: number = 1;

    lastRan: number = 0;

    

    ws!: WebSocket;

    constructor(@Inject(DOCUMENT) document: Document, private backendService: BackendService) { }

    

    ngOnInit(): void {
        this.initWebSocket();
    }

    private initWebSocket(): void {
        this.ws = new WebSocket('ws://'+window.location.hostname+':3000');
        this.ws.onmessage = (evt: MessageEvent) => {this.onNewPixelData(evt)};
        this.ws.onerror = (): void => {
            this.ws.close();
        };
       
        this.ws.onclose = (): void => {
            console.log("live view websocket closed, trying again in a second")

            setTimeout(() => {
                this.initWebSocket();
              }, 1000);
        }
        console.log("Trying to connect to cassette live view websocket")
    }

    private onNewPixelData(evt: MessageEvent): void {
        const pixels: Pixel[] = JSON.parse(evt.data);
        for (let i = 0; i < pixels.length; i++) {
            var pixel_element = document.getElementById("pixel_" + i);
            if(pixel_element != null) {
                if(pixels[i].patched){
                    pixel_element.style.backgroundColor = this.getHex(pixels[i]);
                    // pixel.classList.add("patched");
                    // pixel.classList.remove("unpatched");
                }
                else
                {
                    pixel_element.style.backgroundColor = "transparent";
                    // pixel.classList.add("unpatched");
                    // pixel.classList.remove("patched");
                }
            }
        }
        // let lastRanCopy = this.lastRan
        // this.lastRan = new Date().getTime()
        // console.log(this.lastRan - lastRanCopy);
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
