import { Component, Input, OnInit } from '@angular/core';
import { ResizedEvent } from 'angular-resize-event';
import { Pixel } from 'src/app/pixel';
import { MatrixLiveViewService } from 'src/app/services/matrix-live-view.service';

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

    pixels: Pixel[] = [];


    constructor(private matrixService: MatrixLiveViewService) { }

    ngOnInit(): void {
        this.matrixService.getPixels().subscribe(pixels => {
            // set mat grid tiles to pixel colors
            // for pixel of pixels
            this.pixels = pixels;
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
    }

}
