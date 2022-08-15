import { AfterViewInit, Component, ElementRef, Inject, Input, OnInit, ViewChild } from '@angular/core';
import { DOCUMENT } from '@angular/common';


import { Pixel } from 'src/app/pixel';
import { BackendService } from 'src/app/services/backend.service';


@Component({
  selector: 'live-view',
  templateUrl: './live-view.component.html',
  styleUrls: ['./live-view.component.css']
})
export class LiveViewComponent implements OnInit {
    @Input() n_horizontal: number = 0;
    @Input() n_vertical: number = 0;
    @Input() output_id: number = 0;

    @ViewChild('canvas')
    public canvas!: ElementRef;
    private ctx!: CanvasRenderingContext2D;  
    @ViewChild('canvas_container')
    private canvas_container!: ElementRef;
    

    gutter_size: number = 1;

    pixel_size: number = 5;

    matrix_num: number = 1;

    lastRan: number = 0;

    

    ws!: WebSocket;

    constructor(@Inject(DOCUMENT) document: Document, private backendService: BackendService) { }

    

    ngOnInit(): void {
        this.initWebSocket();
    }

    ngAfterViewInit(): void {
        const canvasEl: HTMLCanvasElement = this.canvas.nativeElement;
        this.ctx = canvasEl.getContext('2d')!;
        
    }

    ngOnDestroy(): void {
        console.log("Destroying matrix live view");
        this.closeWebSocket();
    }

    private initWebSocket(): void {
        console.log("initializing websocket");
        this.closeWebSocket();
        this.ws = new WebSocket('ws://'+window.location.hostname+':8080/ws');
        this.ws.onmessage = (evt: MessageEvent) => {this.onNewPixelData(evt)};
        this.ws.onopen = (): void => {
            this.ws.send(this.output_id.toString());
        }

        this.ws.onerror = (): void => {
            this.closeWebSocket();
        };
       
        this.ws.onclose = (): void => {
            console.log("live view websocket closed, trying again in a second")
            this.closeWebSocket();
            setTimeout(() => {
                this.initWebSocket();
              }, 1000);
        }
        console.log("Trying to connect to cassette live view websocket")
    }

    private closeWebSocket(): void {
        if(this.ws != null){
            this.ws.removeAllListeners?.('open');
            this.ws.removeAllListeners?.('message');
            this.ws.removeAllListeners?.('close');
            this.ws.removeAllListeners?.('error');
            this.ws.close();
        }
    }

    private drawRect(x: number, y: number, width: number, height: number): void {
        this.ctx.fillRect(x, y, width, height);   
    }

    private drawPixel(x: number, y: number, pixel: Pixel): void {
        if(pixel.patched){
            this.ctx.fillStyle = this.getHex(pixel);
        }
        else {
            this.ctx.fillStyle = "transparent";
        }
        this.drawRect(this.gutter_size*x + x*this.pixel_size, this.gutter_size*y + y*this.pixel_size, this.pixel_size, this.pixel_size);
        // console.log("x_ind:", x, "y_indx:", y, "x:", this.gutter_size*x + x*this.pixel_size, " y:",this.gutter_size*y + y*this.pixel_size)
    }

    private onNewPixelData(evt: MessageEvent): void {
        // console.log(evt.data);
        const pixels: Pixel[] = JSON.parse(evt.data);
        // console.log(pixels);
        for (let i = 0; i < pixels.length; i++) {

            this.drawPixel(i % this.n_horizontal, Math.floor(i/this.n_horizontal), pixels[i])
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
