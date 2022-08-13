import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { MatrixEditorRoutingModule } from './matrix-editor-routing.module';
import { MatrixEditorComponent } from './matrix-editor.component';
import { EffectEditorComponent } from './effect-editor/effect-editor.component';
import { ColorPickerDialog } from './effect-editor/effect-editor.component';
import { LiveViewComponent } from '../shared/live-view/live-view.component';
import { MatGridListModule } from '@angular/material/grid-list'; 
import { MatExpansionModule } from '@angular/material/expansion'; 
import { MatIconModule } from '@angular/material/icon';
import { MatRippleModule } from '@angular/material/core';
import { MatSelectModule } from '@angular/material/select';
import { MatSliderModule } from '@angular/material/slider';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { AngularResizeEventModule } from 'angular-resize-event';
import {FormsModule, ReactiveFormsModule} from '@angular/forms';
import { MatInputModule } from '@angular/material/input'; 
import { ColorSketchModule } from 'ngx-color/sketch';
import {MatDialogModule} from '@angular/material/dialog'; 
import { AngularDraggableModule } from 'angular2-draggable';


@NgModule({
  declarations: [
    MatrixEditorComponent,
    LiveViewComponent,
    EffectEditorComponent,
    ColorPickerDialog,
  ],
  imports: [
    CommonModule,
    MatrixEditorRoutingModule,
    MatGridListModule,
    AngularResizeEventModule,
    MatExpansionModule,
    MatIconModule,
    MatRippleModule,
    MatSelectModule,
    MatSliderModule,
    MatCheckboxModule,
    MatCardModule,
    MatButtonModule,
    FormsModule,
    ColorSketchModule,
    MatInputModule,
    ReactiveFormsModule,
    MatDialogModule,
    AngularDraggableModule,
  ],
})
export class MatrixEditorModule { }
