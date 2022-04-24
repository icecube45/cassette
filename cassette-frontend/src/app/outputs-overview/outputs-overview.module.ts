import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { OutputsOverviewRoutingModule } from './outputs-overview-routing.module';
import { OutputsOverviewComponent } from './outputs-overview.component';

import {MatIconModule} from '@angular/material/icon'; 
import {MatButtonModule} from '@angular/material/button'; 
import {MatCardModule} from '@angular/material/card'; 
import {MatListModule} from '@angular/material/list';
import { FlexLayoutModule } from '@angular/flex-layout';


@NgModule({
  declarations: [
    OutputsOverviewComponent
  ],
  imports: [
    CommonModule,
    OutputsOverviewRoutingModule,
    MatIconModule,
    MatButtonModule,
    MatCardModule,
    MatListModule,
    FlexLayoutModule,
  ]
})
export class OutputsOverviewModule { }
