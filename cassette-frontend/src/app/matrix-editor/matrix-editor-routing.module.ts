import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AudioViewComponent } from '../shared/audio-view/audio-view.component';
import { EffectEditorComponent } from './effect-editor/effect-editor.component';
import { MatrixEditorComponent } from './matrix-editor.component';

const routes: Routes = [
    { path: '', component: MatrixEditorComponent },
    { path: 'edit/:type/:id', component: EffectEditorComponent },
    { path: 'lmao', component: AudioViewComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class MatrixEditorRoutingModule { }
