import { DOCUMENT } from '@angular/common';
import { AfterViewInit, Component, ElementRef, Inject, Input, OnInit, ViewChild } from '@angular/core';



// export interface FFT_Bin{
//     value: number;
// }

export interface Frequency_Trigger{
    threshold: number;
    freq_start: number;
    freq_end: number;
}


@Component({
  selector: 'app-audio-view',
  templateUrl: './audio-view.component.html',
  styleUrls: ['./audio-view.component.css']
})
export class AudioViewComponent implements OnInit, AfterViewInit {
    @ViewChild('canvas')
    public canvas!: ElementRef;
    private ctx!: CanvasRenderingContext2D;  

    gutter: number = 1;

    matrix_num: number = 1;

    lastRan: number = 0;

    
    frequency_triggers: Frequency_Trigger[] = [
        {threshold: 0.5, freq_start: 0, freq_end: 1000},
        {threshold: 0.25, freq_start: 1000, freq_end: 4000},
        {threshold: 0.75, freq_start: 4000, freq_end: 4500},
        {threshold: 0.1, freq_start: 6000, freq_end: 8000},
        {threshold: 0.2, freq_start: 9000, freq_end: 12000},
        {threshold: 0.4, freq_start: 12000, freq_end: 12300}
    ]
    ws!: WebSocket;

    constructor(@Inject(DOCUMENT) document: Document) { }

    

    ngOnInit(): void {
        this.initWebSocket();
    }
    
    ngAfterViewInit(): void {
        const canvasEl: HTMLCanvasElement = this.canvas.nativeElement;
        this.ctx = canvasEl.getContext('2d')!;
    }

    private initWebSocket(): void {
        this.ws = new WebSocket('ws://'+window.location.hostname+':8080/dsp_ws');
        this.ws.onmessage = (evt: MessageEvent) => {this.onNewFFTData(evt)};
        this.ws.onopen = (): void => {
            console.log("Connected to websocket");
        }

        this.ws.onerror = (): void => {
            this.ws.close();
        };
       
        this.ws.onclose = (): void => {
            console.log("FFT websocket closed, trying again in a second")

            setTimeout(() => {
                this.initWebSocket();
              }, 1000);
        }
        console.log("Trying to connect to cassette FFT websocket")
    }
    private drawRect(x: number, y: number, width: number, height: number): void {
        this.ctx.fillRect(x, y, width, height);

        
    }


    private onNewFFTData(evt: MessageEvent): void {
        console.log(evt.data);
        // const bins: FFT_Bin[] = JSON.parse(evt.data);
        const bin_msg = JSON.parse(evt.data);
        const bins = bin_msg.bins;

        // clear the canvas
        this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
        if(bin_msg.beat) {
            this.ctx.fillStyle = '#FFFF00';
            this.ctx.fillRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
        }
        // get length of bins and determine pixel size
        let bin_size = (this.ctx.canvas.width - (bins.length-1)*this.gutter) / bins.length;
        for (let i = 0; i < bins.length; i++) {
            this.ctx.fillStyle = '#00' + (bins[i]*128+128).toString(16).split('.')[0].padStart(2, '0') + '00';

            


            // console.log((bins[i]*128).toString(16).split('.')[0].padStart(2, '0'));
            let bin_height = bins[i]*this.ctx.canvas.height;
            this.drawRect(i*this.gutter + i * bin_size, this.ctx.canvas.height-bin_height, bin_size, bin_height);
            // console.log(bin_size);
            // console.log(bins[i].value);
            // for (let x = 0; x < 10; x++) {
            //     this.ctx.clearRect(0, 0, this.canvas.nativeElement.width, this.canvas.nativeElement.height);
            //     this.draw(x, 10, 10);
            //   }
        }
        for (let i = 0; i < this.frequency_triggers.length; i++) {
            const freq_trigger = this.frequency_triggers[i];
            this.ctx.strokeStyle = '#000000';
            let trigger_x_pixels = this.ctx.canvas.width/(bin_msg.max-bin_msg.min)*freq_trigger.freq_start
            let trigger_width_pixels = this.ctx.canvas.width/(bin_msg.max-bin_msg.min)*(freq_trigger.freq_end-freq_trigger.freq_start)
            this.ctx.strokeRect(trigger_x_pixels, 0, trigger_width_pixels, this.ctx.canvas.height);
            // draw a line at the trigger point
            this.ctx.strokeStyle = '#FF0000';
            this.ctx.beginPath();
            this.ctx.moveTo(trigger_x_pixels, this.ctx.canvas.height-this.frequency_triggers[i].threshold*this.ctx.canvas.height);
            this.ctx.lineTo(trigger_x_pixels+trigger_width_pixels, this.ctx.canvas.height-this.frequency_triggers[i].threshold*this.ctx.canvas.height);
            this.ctx.stroke();
            //convert freq_start and freq_end to closest bin
            const freq_start_bin = Math.round((freq_trigger.freq_start-bin_msg.min)/(bin_msg.max-bin_msg.min)*bins.length);
            const freq_end_bin = Math.round((freq_trigger.freq_end-bin_msg.min)/(bin_msg.max-bin_msg.min)*bins.length);

            // for each bin in range of freq_start and freq_end, compute average
            let avg = 0;
            for (let i = freq_start_bin; i < freq_end_bin; i++) {
                avg += bins[i];
            }
            avg /= (freq_end_bin-freq_start_bin);
            this.ctx.fillStyle = '#0000FF';
            this.ctx.globalAlpha = 0.45;
            if(avg > freq_trigger.threshold) {
                this.ctx.fillStyle = '#FF0000';
                this.ctx.globalAlpha = 0.75;
            }
            // draw a rectangle at the average
            this.drawRect(trigger_x_pixels, this.ctx.canvas.height-avg*this.ctx.canvas.height, trigger_width_pixels, avg*this.ctx.canvas.height);
            this.ctx.globalAlpha = 1.0;
        }

        this.ctx.fillStyle = '#000000';
        this.ctx.fillText(bin_msg.min, 10, this.ctx.canvas.height-10);
        this.ctx.fillText(bin_msg.max, this.ctx.canvas.width-30, this.ctx.canvas.height-10);
        // let lastRanCopy = this.lastRan
        // this.lastRan = new Date().getTime()
        // console.log(this.lastRan - lastRanCopy);
    }






}
