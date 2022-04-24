import { Component, OnInit } from '@angular/core';
import { Output } from '../output';
import { OUTPUTS } from '../mock-outputs';
import { Router } from '@angular/router'; 

@Component({
  selector: 'app-outputs-overview',
  templateUrl: './outputs-overview.component.html',
  styleUrls: ['./outputs-overview.component.css']
})
export class OutputsOverviewComponent implements OnInit {

    outputs = OUTPUTS;

    constructor(private router: Router,) { }


    onConfigure(output: Output): void {
        console.log(`Configure ${output.name}`);
        this.router.navigateByUrl('/output/' + output.id);
    }

    ngOnInit(): void {
    }

}
