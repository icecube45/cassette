import { Injectable } from "@angular/core";
import { map, Observable, of, Subject, switchMap } from "rxjs";
import { webSocket } from 'rxjs/webSocket';
import { Pixel } from "../pixel";

export interface PixelInterface {
    r: number;
    g: number;
    b: number;
}


@Injectable({
    providedIn: 'root'
})
export class MatrixLiveViewService {
    private socket  = webSocket<PixelInterface[]>("ws://localhost:3000");
    public messages = this.socket.asObservable();
    
    private pixels_subject = new Subject<Pixel[]>();

    constructor() { 
        console.log("matrixLiveViewService");

        this.socket.subscribe({ 
            next: (msg) => {
                let new_pixels: Pixel[] = [];
                for(let pixel of msg) {
                    new_pixels.push(new Pixel(pixel.r, pixel.g, pixel.b));
                }
                this.pixels_subject.next(new_pixels);
            },
            error: (err) => {
                console.log(err);
            },
            complete: () => {
                console.log("complete");
            }
        });

        // this.getPixels().subscribe(pixels => {
        //     console.log("got new pixels - service");
        // }
        // );
    }


    public sendMessage(msg: any) {
      this.socket.next(msg);
    }

    public getPixels(): Observable<Pixel[]> {       
        return this.pixels_subject.asObservable();
    }
}