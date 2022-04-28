import { Injectable } from "@angular/core";
import { map, Observable, of, retry, Subject, switchMap } from "rxjs";
import { Pixel } from "../pixel";
import { HttpClient } from "@angular/common/http";



@Injectable({
    providedIn: 'root'
})
export class BackendService {

    
    // private pixels = this.socket.fromEvent<string>('pixels');


    constructor() { 
        console.log("matrixLiveViewService");

        // this.socket.pipe(retry()).subscribe({ 
        //     next: (msg) => {
        //         // let new_pixels: Pixel[] = [];
        //         // for(let pixel of msg) {
        //         //     new_pixels.push(new Pixel(pixel.r, pixel.g, pixel.b, pixel.patched));
        //         // }
        //         // this.pixels_subject.next(new_pixels);
        //     },
        //     error: (err) => {
        //         console.log(err);
        //     },
        //     complete: () => {
        //         console.log("complete");
        //     }
        // });

        // this.getPixels().subscribe(pixels => {
        //     console.log("got new pixels - service");
        // }
        // );
    }


    public sendMessage(msg: any) {
    //   this.socket.next(msg);
    }

    // public getPixels(): Observable<string> {       
        // return this.pixels_subject.asObservable();
        // return this.messages.pipe(retry());
        // return this.socket.fromEvent<string>('pixels');
    // }
}