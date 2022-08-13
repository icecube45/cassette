import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router'
import { AngularDraggableModule } from 'angular2-draggable';

@Component({
  selector: 'app-matrix-editor',
  templateUrl: './matrix-editor.component.html',
  styleUrls: ['./matrix-editor.component.css']
})
export class MatrixEditorComponent implements OnInit {

  public output_id: number = 0;
  
  constructor(private route: ActivatedRoute) { }

  ngOnInit(): void {
    this.output_id = Number(this.route.snapshot.paramMap.get('output_id'));

  }

}
