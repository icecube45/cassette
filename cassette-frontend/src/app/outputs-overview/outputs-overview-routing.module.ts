import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { OutputsOverviewComponent } from './outputs-overview.component';



const routes: Routes = [
    { path: '', component: OutputsOverviewComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class OutputsOverviewRoutingModule { }
