import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';



const routes: Routes = [
    {
        path: "output/:id",
        loadChildren: () => import('./matrix-editor/matrix-editor.module').then(m => m.MatrixEditorModule)
    },
    {
        path: "home",
        loadChildren: () => import("./outputs-overview/outputs-overview.module").then(m => m.OutputsOverviewModule)
    },
    { path: '', redirectTo: 'home', pathMatch: 'full'}
];

@NgModule({
    imports: [RouterModule.forChild(routes)],
    exports: [RouterModule]
})

export class AppRoutingModule { }