import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { EffectEditorComponent } from './effect-editor/effect-editor.component';
import { MatrixEditorComponent } from './matrix-editor.component';

const routes: Routes = [
    { path: '', component: MatrixEditorComponent },
    { path: 'edit/channel/:channel_id', component: EffectEditorComponent },
    { path: 'edit/mixer/:mixer_id', component: EffectEditorComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class MatrixEditorRoutingModule { }
