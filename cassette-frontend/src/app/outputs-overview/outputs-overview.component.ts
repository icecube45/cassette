import { Component, OnInit } from '@angular/core';
import { Output } from '../output';
import { BackendService } from 'src/app/services/backend.service';

import { Router } from '@angular/router'; 

@Component({
  selector: 'app-outputs-overview',
  templateUrl: './outputs-overview.component.html',
  styleUrls: ['./outputs-overview.component.css']
})
export class OutputsOverviewComponent implements OnInit {

    
    constructor(private router: Router, private backend: BackendService) { }
    
    outputs: Output[] = [];

    onConfigure(output: Output): void {
        console.log(`Configure ${output.name}`);
        this.router.navigateByUrl('/output/' + output.id);
    }

    ngOnInit(): void {
        this.backend.getOutputs().subscribe(outputs => {
            this.outputs = outputs
        });
    }

}
