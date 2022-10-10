import { Injectable } from "@angular/core";
import { map, Observable, of, retry, Subject, switchMap } from "rxjs";
import { Pixel } from "../pixel";
import { HttpClient } from "@angular/common/http";
import { EFFECTS } from "../mock-effects";
import { MIXER } from '../mock-mixers';
import { Mixer } from "../mixer";
import { Effect, EffectsWrapper } from "../effect";


@Injectable({
    providedIn: 'root'
})
export class BackendService {

    
    // private pixels = this.socket.fromEvent<string>('pixels');


    constructor(private http: HttpClient) { 
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

    public getMixer(outputID: number, mixerID: number): Observable<Mixer> {
        return of(MIXER);
        return this.http.get<Mixer>(`/api/output/${outputID}/mixers/${mixerID}`);
    }

    public getEffects(outputID: number, channelID: number): Observable<EffectsWrapper> {
        return of(EFFECTS);
        return this.http.get<EffectsWrapper>(`/api/output/${outputID}/channels/${channelID}/effects`);
    }

    public updateChannelEffectOptions(outputID: number, channelID: number, effect: Effect): Observable<Effect> {
        return this.http.post<Effect>(`/api/output/${outputID}/channels/${channelID}/effects/${effect.id}`, effect);
    }

    public setChannelActiveEffect(outputID: number, channelID: number, effect_id: number): Observable<Effect> {
        return this.http.post<Effect>(`/api/output/${outputID}/channels/${channelID}/active_effect`, effect_id);
    }

    public updateMixer(outputID: number, mixerID: number, mixer: Mixer): Observable<Mixer> {
        return this.http.post<Mixer>(`/api/output/${outputID}/mixers/${mixerID}`, mixer);
    }

    // public getPixels(): Observable<string> {       
        // return this.pixels_subject.asObservable();
        // return this.messages.pipe(retry());
        // return this.socket.fromEvent<string>('pixels');
    // }
}